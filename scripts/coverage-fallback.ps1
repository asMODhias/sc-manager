# Coverage fallback: merge .profraw into merged.profdata (if llvm-profdata present) and zip artifacts
# Does not fail CI: best-effort to collect coverage artifacts for offline analysis

param()

$ErrorActionPreference = 'Stop'

Write-Host "==> Coverage fallback: looking for .profraw files"
$profraws = Get-ChildItem -Path .. -Recurse -Filter *.profraw -ErrorAction SilentlyContinue | Select-Object -ExpandProperty FullName -ErrorAction SilentlyContinue
if (-not $profraws -or $profraws.Count -eq 0) {
    Write-Host "No .profraw files found; nothing to collect"
    exit 0
}

$timestamp = (Get-Date).ToString('yyyyMMdd_HHmmss')
$artifactDir = Join-Path -Path .. -ChildPath "artifacts"
if (-not (Test-Path $artifactDir)) { New-Item -ItemType Directory -Path $artifactDir | Out-Null }
$artifactPath = Join-Path -Path $artifactDir -ChildPath "coverage-fallback-$timestamp.zip"

# Check for llvm-profdata
$profdataCmd = Get-Command llvm-profdata -ErrorAction SilentlyContinue
if (-not $profdataCmd) {
    Write-Host "llvm-profdata not found in PATH; zipping profraw files only"
    Compress-Archive -Path $profraws -DestinationPath $artifactPath -Force
    Write-Host "Wrote coverage artifact: $artifactPath"
    exit 0
}

# Merge profraws
$tempMerged = Join-Path -Path $env:TEMP -ChildPath "merged_$timestamp.profdata"
$profrawList = $profraws -join ' '
Write-Host "Merging profraw files into: $tempMerged"
$mergeArgs = @('merge', '-sparse', '-o', $tempMerged) + $profraws
try {
    & llvm-profdata @mergeArgs
} catch {
    Write-Warning "llvm-profdata merge failed: $_"
    Write-Host "Zipping profraws as fallback"
    Compress-Archive -Path $profraws -DestinationPath $artifactPath -Force
    Write-Host "Wrote coverage artifact: $artifactPath"
    exit 0
}

# Create artifact with merged profdata and original profraws
$tempFiles = $profraws + $tempMerged
Compress-Archive -Path $tempFiles -DestinationPath $artifactPath -Force
Write-Host "Wrote coverage artifact: $artifactPath"

# Keep merged file next to artifacts for convenience
Move-Item -Path $tempMerged -Destination $artifactDir -Force
Write-Host "Moved merged profile to: $(Join-Path $artifactDir (Split-Path $tempMerged -Leaf))"

exit 0
