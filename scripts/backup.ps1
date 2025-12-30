# Backup workspace and tag
param(
    [string]$Tag = "v7.1.1-final-backup",
    [string]$OutDir = "./backups"
)

Write-Host "Creating backup archive..."
if (-not (Test-Path $OutDir)) { New-Item -ItemType Directory -Path $OutDir | Out-Null }

$now = Get-Date -Format "yyyyMMdd_HHmmss"
$archive = Join-Path $OutDir "backup_$now.zip"

# exclude large or CI artifacts by default
$exclude = @('node_modules','target','.git')
$items = Get-ChildItem -Path . -Force | Where-Object { $exclude -notcontains $_.Name }

Compress-Archive -Path ($items | ForEach-Object { $_.FullName }) -DestinationPath $archive -Force
Write-Host "Archive created: $archive"

# Create an annotated lightweight tag locally
Write-Host "Creating git tag: $Tag"
git tag -a $Tag -m "Pre V8 backup $now"
Write-Host "Tag created (local). To push: git push origin $Tag"
