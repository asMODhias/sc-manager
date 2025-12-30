# Clean common build artifacts and node_modules
Write-Host "Cleaning workspace: removing node_modules and target directories"

Get-ChildItem -Path . -Directory -Recurse -Force -ErrorAction SilentlyContinue | Where-Object { $_.Name -in @('node_modules','target') } | ForEach-Object {
    try {
        Write-Host "Removing: $($_.FullName)"
        Remove-Item -LiteralPath $_.FullName -Recurse -Force -ErrorAction Stop
    } catch {
        Write-Warning "Failed to remove $($_.FullName): $_"
    }
}

Write-Host "Cleanup completed."