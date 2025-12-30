<#
Runs the project's local CI and, when it passes, merges a PR via admin override.
Usage: ./scripts/auto-merge-local-ci.ps1 -PRNumber 28
Requirements:
 - gh CLI authenticated with permissions to merge via admin override
 - Local CI scripts working: ./scripts/run-local-ci.ps1
#>
param(
    [int]$PRNumber = 28
)

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
