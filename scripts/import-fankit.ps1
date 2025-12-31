<#
.SYNOPSIS
  Import CIG Fankit assets into the repo.
.SYNTAX
  ./import-fankit.ps1 -SourcePath <path>
#>
param(
  [Parameter(Mandatory=$true)]
  [string]$SourcePath
)

$Dest = "assets/fankit"
$IncludeExts = @('png','jpg','jpeg','svg','webp','gif','mp3','ogg','wav','txt','md','json','yml','yaml')

if (-not (Test-Path $SourcePath -PathType Container)) {
  Write-Error "Source path does not exist: $SourcePath"; exit 2
}

# find license/tos
$tos = Get-ChildItem -Path $SourcePath -Recurse -File -Depth 3 | Where-Object { $_.Name -match '(?i)license|tos' } | Select-Object -First 1
if (-not $tos) {
  Write-Error "ERROR: LICENSE/TOS not found in source folder. Place it in the source before running the import."; exit 3
}

New-Item -ItemType Directory -Force -Path $Dest | Out-Null
Copy-Item -Path $tos.FullName -Destination "$Dest\LICENSE_CIG_FANKIT.txt" -Force

$manifest = @()

Get-ChildItem -Path $SourcePath -Recurse -File | ForEach-Object {
  $ext = $_.Extension.TrimStart('.')
  if ($IncludeExts -contains $ext) {
    $rel = Resolve-Path -Relative -Path $_.FullName -RelativeTo $SourcePath 2>$null
    if (-not $rel) { $rel = $_.FullName.Substring($SourcePath.Length).TrimStart('\') }
    $destPath = Join-Path $Dest $rel
    New-Item -ItemType Directory -Force -Path (Split-Path $destPath) | Out-Null
    Copy-Item -Path $_.FullName -Destination $destPath -Force
    $sha = (Get-FileHash -Path $destPath -Algorithm SHA256).Hash
    $size = (Get-Item $destPath).Length
    $manifest += [PSCustomObject]@{
      path = $rel
      size = $size
      sha256 = $sha
      source = $SourcePath
      imported_at = (Get-Date).ToString('o')
    }
  }
}

$manifest | ConvertTo-Json -Depth 5 | Out-File -FilePath "$Dest\fankit-manifest.json" -Encoding utf8

# Append changelog
$changelog = "\n## Import: $(Get-Date -Format o)\n- Quelle: $SourcePath\n- Dateianzahl importiert: $($manifest.Count)\n- Lizenz: assets/fankit/LICENSE_CIG_FANKIT.txt\n"
Add-Content -Path "docs/CIG-Fankit.md" -Value $changelog

Write-Host "Imported $($manifest.Count) files to $Dest"
Write-Host "Manifest written to $Dest\fankit-manifest.json"
Write-Host "License copied to $Dest\LICENSE_CIG_FANKIT.txt"
Write-Host "Done. Please review and commit: git add $Dest && git commit -m 'Add CIG Fankit assets (import) + manifest + LICENSE'"
