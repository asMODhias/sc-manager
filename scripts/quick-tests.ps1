# Quick script to run unit tests for the Rust workspace and JS tests locally
# This script is robust: it runs workspace tests when a root Cargo.toml exists
# and only runs UI/E2E tests if the corresponding folders exist.

# Run Rust tests across the workspace when possible
if (Test-Path -Path "./Cargo.toml") {
    Write-Host "Running Rust tests (workspace)..."
    cargo test --workspace
} else {
    Write-Host "No workspace Cargo.toml found - skipping Rust workspace tests."
}

# UI tests (npm)
if (Test-Path -Path "./ui/package.json") {
    Write-Host "Running UI tests (npm)..."
    Push-Location ui
    npm ci
    npm test
    Pop-Location
} else {
    Write-Host "Skipping UI tests: ./ui not found or no package.json."
}

# E2E tests (Playwright)
if (Test-Path -Path "./e2e/package.json") {
    Write-Host "Running E2E tests (playwright)..."
    Push-Location e2e
    npm ci
    npx playwright test
    Pop-Location
} else {
    Write-Host "Skipping E2E tests: ./e2e not found or no package.json."
}

Write-Host "All tests finished."