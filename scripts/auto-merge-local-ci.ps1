<#
Runs the project's local CI and, when it passes, merges a PR via admin override.
Usage: ./scripts/auto-merge-local-ci.ps1 -PRNumber 28
Requirements:
 - gh CLI authenticated with permissions to merge via admin override
 - Local CI scripts working: ./scripts/run-local-ci.ps1
#>
param(
    [int]$PRNumber = 28,
    [switch]$AutoInstallProtoc = $false
)

# Ensure we run from repository root (parent of scripts directory)
$RepoRoot = Resolve-Path (Join-Path $PSScriptRoot '..')
Set-Location $RepoRoot

# Preflight: check required tools
if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
    Write-Host "[auto-merge] gh CLI not found in PATH. Please install and authenticate gh before running this script." -ForegroundColor Red
    exit 1
}

# Check local CI script exists
if (-not (Test-Path "$RepoRoot\scripts\run-local-ci.ps1")) {
    Write-Host "[auto-merge] run-local-ci.ps1 not found in scripts directory. Ensure you're in the repository root." -ForegroundColor Red
    exit 1
}

# Check for protoc (required by some crates) or PROTOC env var
$protocPath = $env:PROTOC
if (-not $protocPath) {
    $protocPath = (Get-Command protoc -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -ErrorAction SilentlyContinue)
}
if (-not $protocPath) {
    if ($AutoInstallProtoc) {
        Write-Host "[auto-merge] protoc not found — attempting automatic installation (choco/scoop) ..." -ForegroundColor Yellow
        if (Get-Command choco -ErrorAction SilentlyContinue) {
            Write-Host "[auto-merge] Installing protoc via choco (may require admin privileges)..."
            choco install protoc -y
        } elseif (Get-Command scoop -ErrorAction SilentlyContinue) {
            Write-Host "[auto-merge] Installing protoc via scoop..."
            scoop install protoc
        } else {
            Write-Host "[auto-merge] No supported Windows package manager (choco or scoop) found to install protoc." -ForegroundColor Red
            Write-Host "Please install protoc manually or set PROTOC to the protoc binary path. See docs/LOCAL_AUTO_MERGE.md" -ForegroundColor Red
            exit 2
        }
        # re-check
        $protocPath = (Get-Command protoc -ErrorAction SilentlyContinue | Select-Object -ExpandProperty Source -ErrorAction SilentlyContinue)
        if (-not $protocPath) {
            Write-Host "[auto-merge] protoc installation attempt did not make protoc available in PATH. See docs/LOCAL_AUTO_MERGE.md for manual steps." -ForegroundColor Red
            exit 2
        }
    } else {
        Write-Host "[auto-merge] protoc not found. Some crates require protoc to build (prost)." -ForegroundColor Yellow
        Write-Host "Either install protoc and add to PATH, or set the PROTOC environment variable to the protoc binary path." -ForegroundColor Yellow
        Write-Host "Use -AutoInstallProtoc to attempt an automatic installation, or see docs/LOCAL_AUTO_MERGE.md for manual instructions." -ForegroundColor Yellow
        exit 2
    }
}

Write-Host "[auto-merge] Running local CI..."
& "$PSScriptRoot\run-local-ci.ps1"
$exit = $LASTEXITCODE
if ($exit -ne 0) {
    Write-Host "[auto-merge] Local CI failed with exit code $exit. Aborting merge." -ForegroundColor Red
    exit $exit
}

Write-Host "[auto-merge] Local CI passed. Posting PR comment and merging (#$PRNumber)..."
$comment = "Auto-merge: Local CI passed; merging via admin override as per local-only CI policy."
gh pr comment $PRNumber --body $comment
# Perform admin override merge and delete branch
gh pr merge $PRNumber --merge --delete-branch --admin
if ($LASTEXITCODE -eq 0) {
    Write-Host "[auto-merge] PR #$PRNumber merged and branch deleted." -ForegroundColor Green
} else {
    Write-Host "[auto-merge] Merge command failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}
