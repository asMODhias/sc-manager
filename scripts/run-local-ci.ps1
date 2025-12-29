# Local CI for PowerShell
$ErrorActionPreference = 'Stop'

Write-Host "==> Formatting check"
cargo fmt -- --check

Write-Host "==> Clippy"
cargo clippy --workspace --all-targets -- -D warnings

Write-Host "==> Run core tests"
Push-Location core
cargo test --verbose
Pop-Location

Write-Host "==> Run adapters tests"
Push-Location adapters
cargo test --verbose
Pop-Location

Write-Host "==> Run app integration tests"
Push-Location app
cargo test --test integration --verbose
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

Write-Host "Local CI finished."