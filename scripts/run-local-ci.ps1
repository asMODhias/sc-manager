# Local CI for PowerShell
$ErrorActionPreference = 'Stop'

Write-Host "==> Formatting check (per-crate)"
Get-ChildItem -Path . -Recurse -Filter Cargo.toml | Where-Object { $_.FullName -notmatch '\\patches\\|\\artifacts\\' } | ForEach-Object {
    $manifest = $_.FullName
    Write-Host "Formatting $manifest"
    cargo fmt --manifest-path $manifest -- --check
}

Write-Host "==> Preflight: protoc check"
# Check for protoc binary or PROTOC env var; if missing, skip crates that require protoc (prost)
$protocFound = $false
if ($env:PROTOC) {
    if (Test-Path $env:PROTOC) { $protocFound = $true }
} else {
    if (Get-Command protoc -ErrorAction SilentlyContinue) { $protocFound = $true }
}
if (-not $protocFound) {
    Write-Host "[warn] protoc not found. Crates that require `protoc` (prost) will be skipped. Install protoc or set PROTOC env var to enable them." -ForegroundColor Yellow
    $skipProtoc = $true
} else {
    $skipProtoc = $false
}

Write-Host "==> Clippy (per-crate)"
Get-ChildItem -Path . -Recurse -Filter Cargo.toml | Where-Object { $_.FullName -notmatch '\\patches\\|\\artifacts\\' } | ForEach-Object {
    $manifest = $_.FullName
    if ($skipProtoc -and $manifest -match 'p2p') {
        Write-Host "Skipping $manifest (requires protoc)"
        return
    }
    Write-Host "Clippy $manifest"
    cargo clippy --manifest-path $manifest --all-targets -- -D warnings
}

Write-Host "==> Run core tests"
Push-Location core
cargo test --verbose
Pop-Location

Write-Host "==> Run adapters tests"
Push-Location adapters
cargo test --verbose
Pop-Location

Write-Host "==> Run app tests (unit + integration)"
Push-Location app
cargo test --verbose
Pop-Location

if (Test-Path -Path "./ui/package.json") {
    Write-Host "==> UI tests"
    Push-Location ui
    npm ci
    npm test --if-present
    Pop-Location
} else {
    Write-Host "==> Skipping UI tests: no ui/package.json"
}

if (Test-Path -Path "./e2e/package.json") {
    Write-Host "==> E2E: prepare fixtures"
    python scripts/anonymize_game_log.py --in e2e/fixtures/raw_game.log --out e2e/fixtures/raw_game.anonymized.log --seed=local
    python scripts/anonymize_game_log.py --in e2e/fixtures/fleetyards_export.csv --out e2e/fixtures/fleetyards_export.anonymized.csv --seed=local
    python scripts/anonymize_game_log.py --in e2e/fixtures/erkul_dump.json --out e2e/fixtures/erkul_dump.anonymized.json --seed=local
    python scripts/import_fixtures.py --game e2e/fixtures/raw_game.anonymized.log --fleet e2e/fixtures/fleetyards_export.anonymized.csv --erkul e2e/fixtures/erkul_dump.anonymized.json --out e2e/testdata/testdata.json
    
    Write-Host "==> Install Playwright browsers"
    Push-Location e2e
    npx playwright install --with-deps
    npx playwright test --reporter=dot
    Pop-Location
} else {
    Write-Host "==> Skipping E2E tests: no e2e/package.json"
}

Write-Host "==> Coverage fallback"
& "$PSScriptRoot\coverage-fallback.ps1"

Write-Host "Local CI finished."