## Summary
This PR adds the Cloud Imperium Games (CIG) Fankit assets and tooling:
- `assets/fankit/` — imported assets (logos, wallpapers, fonts, audio snippets, guidelines and license)
- `assets/fankit/fankit-manifest.json` — generated manifest (path, size, sha256, source, imported_at)
- `assets/fankit/LICENSE_CIG_FANKIT.txt` — copy of the original license/TOS
- `scripts/import-fankit.*` — import scripts (bash and PowerShell)
- `scripts/check-fankit-manifest.sh` — manifest validation that runs in CI
- `docs/CIG-Fankit.md` — documentation (TOS, update path, usage notes)
- `docs/examples/CIG-Fankit-Demo.md` — demo usage page referencing assets
- `.github/workflows/fankit-manifest-check.yml` — CI job to validate manifest & license on changes

## License/TOS
- The included `LICENSE_CIG_FANKIT.txt` is a verbatim copy of the license found in the source Fankit.
- Ensure you have the right to include these specific files in the repo before merging; if any file is not permitted, it must be removed and the manifest updated.

## Checklist
- [ ] Confirm legal review (if required)
- [ ] Verify `scripts/check-fankit-manifest.sh` runs in CI successfully
- [ ] Validate we didn't accidentally commit files the license forbids (e.g., source files like PSD/AI) — see `assets/fankit/README.md`

## How to update
To update the fankit from a local source folder:
```bash
./scripts/import-fankit.sh /path/to/Fankit_Source
# or on Windows (PowerShell)
./scripts/import-fankit.ps1 -SourcePath "C:\Path\To\Fankit"
```
This will re-generate `assets/fankit/fankit-manifest.json` and append a changelog entry to `docs/CIG-Fankit.md`.

---
*If you want me to open the PR on GitHub (draft), say **Open PR** and I will create the branch and attempt to open a draft PR for review.*