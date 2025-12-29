$files = Get-ChildItem -Path artifacts/coverage -Recurse -Filter cobertura.xml
foreach ($f in $files) {
    $crate = Split-Path $f.DirectoryName -Leaf
    $xml = [xml](Get-Content $f.FullName)
    $classes = @()
    if ($xml.coverage -and $xml.coverage.packages -and $xml.coverage.packages.package) {
        foreach($pkg in $xml.coverage.packages.package) {
            if ($pkg.classes -and $pkg.classes.class) {
                foreach($cl in $pkg.classes.class) {
                    $file = $cl.filename
                    $lr = $cl.'line-rate'
                    if(-not $lr) { $lr = $cl.'@line-rate' }
                    $rate = 0.0
                    if ($lr) { [double]::TryParse($lr,[ref]$rate) | Out-Null }
                    $classes += [pscustomobject]@{ crate=$crate; file=$file; rate=$rate }
                }
            }
        }
    }
    Write-Output "\n== $crate =="
    $classes | Sort-Object -Property rate | Select-Object -First 10 | ForEach-Object { '{0,-30} {1,6:N1}%' -f $_.file, ($_.rate*100) }
}