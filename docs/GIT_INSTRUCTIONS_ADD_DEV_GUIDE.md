# Branch & PR instructions — Add DEV Guide (for maintainers)

Use these commands locally to create a branch, commit changes and push to origin.

```bash
# create branch
git checkout -b feature/docs/copilot-dev-guide

# add files
git add docs/04_DEV_GUIDE_COPILOT.md docs/04_DEV_GUIDE_COPILOT.de.md docs/adr/ADR-0001-COPILOT-DEV-GUIDE.md docs/PR_DRAFT_COPILOT_DEV_GUIDE.md scripts/check-dev-guide.sh .github/workflows/dev-guide-check.yml .github/PULL_REQUEST_TEMPLATE.md

# commit
git commit -m "docs: add COPILOT dev guide (EN + DE), ADR, PR draft and CI check"

# push branch
git push -u origin feature/docs/copilot-dev-guide

# create PR (using hub, gh or UI)
# gh example:
# gh pr create --fill --base main --head feature/docs/copilot-dev-guide
```

Notes:
- Ensure CI passes and request reviewers from core maintainers.
- If approved, merge with squash or merge commit per repo policy.
