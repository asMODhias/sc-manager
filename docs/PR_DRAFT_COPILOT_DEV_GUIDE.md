# PR Draft: Add DEV Guide (COPILOT) + German translation + ADR

Branch: feature/docs/copilot-dev-guide

## Summary
Add a concise developer guide derived from `feature/sot/COPILOT_MASTER_INSTRUCTION_V5_FINAL_OPTIMIZED.md`, add a German translation and create ADR-0001 to document the decision.

## Changes
- Add `docs/04_DEV_GUIDE_COPILOT.md` (EN) — already committed
- Add `docs/04_DEV_GUIDE_COPILOT.de.md` (DE)
- Add `docs/adr/ADR-0001-COPILOT-DEV-GUIDE.md`
- Add this PR draft `docs/PR_DRAFT_COPILOT_DEV_GUIDE.md`

## Checklist
- [ ] Content reviewed by core maintainers
- [ ] ADR approved
- [ ] Spellcheck & formatting done
- [ ] CI checks pass (biome, cargo-audit)
- [ ] Add to release notes if accepted

## Tests
- No code changes; add to release notes and request maintainers to confirm acceptance.

## Notes for reviewer
- The EN dev guide is a concise summary; the original authoritative doc remains under `feature/sot` and should be referenced for full details.
- If accepted, I recommend adding a CI check that enforces the Pre-Code Checklist via PR template or a CI job.

---

Suggested commit title: `docs: add COPILOT dev guide (EN + DE) and ADR`
Suggested PR description: Use this file as the body and link ADR & master doc.
