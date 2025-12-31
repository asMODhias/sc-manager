param()
$Dest = "assets/fankit"
$Manifest = Join-Path $Dest "fankit-manifest.json"
if (-not (Test-Path $Dest -PathType Container)) { Write-Error "$Dest not found"; exit 2 }

# find license
$license = Get-ChildItem -Path $Dest -Recurse -File -Depth 2 | Where-Object { $_.Name -match '(?i)license|tos' } | Select-Object -First 1
if ($license) { Write-Host "Found license: $($license.Name)" } else { Write-Host "WARNING: License/TOS not found inside $Dest" }

$includeExts = 'png','jpg','jpeg','svg','webp','gif','mp3','ogg','wav','txt','md','json','yml','yaml','pdf'

$entries = @()
Get-ChildItem -Path $Dest -Recurse -File | ForEach-Object {
  if ($_.FullName -eq (Resolve-Path $Manifest)) { return }
  $rel = $_.FullName.Substring((Get-Item $Dest).FullName.Length).TrimStart('\')
  $ext = $_.Extension.TrimStart('.')
  $include = $false
  if ($_.Name -ieq 'README.md' -or $_.Name -match '(?i)license') { $include = $true }
  elseif ($includeExts -contains $ext) { $include = $true }

  if ($include) {
    $sha = (Get-FileHash -Path $_.FullName -Algorithm SHA256).Hash
    $size = (Get-Item $_.FullName).Length
    $entries += [PSCustomObject]@{
      path = $rel
      size = $size
      sha256 = $sha
      source = 'local-import'
      imported_at = (Get-Date).ToString('o')
    }
  }
}

$entries | ConvertTo-Json -Depth 5 | Out-File -FilePath $Manifest -Encoding utf8

# changelog
$now = (Get-Date).ToString('o')
$num = $entries.Count
$licensePath = if ($license) { $license.FullName } else { 'not-found' }
$changelog = "`n`n## Manifest generated: $now`n- Quelle: local-import (added directly to repo under assets/fankit)`n- Dateianzahl aufgeführt im Manifest: $num`n- Lizenz: $licensePath`n"
Add-Content -Path "docs/CIG-Fankit.md" -Value $changelog

Write-Host "Manifest generated at $Manifest"
Write-Host "Entries: $num"
Write-Host "Done."