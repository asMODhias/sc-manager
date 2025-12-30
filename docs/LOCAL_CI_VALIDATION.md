# Local CI Validation Guide

Kurz: Diese Anleitung zeigt, wie Entwickler die lokale GitHub Actions Docker‑CI (via `act`) und die PowerShell/Unix‑Local CI Scripte ausführen, um sicherzustellen, dass PRs sauber sind bevor gepusht wird.

## Schnellbefehle

- Windows (PowerShell):
  - Voller Lauf: `.	ools\run-local-ci.ps1` (oder `.































Diese Datei ist zur schnellen Orientierung gedacht; ich kann gerne eine ausführlichere Troubleshooting‑Sektion hinzufügen oder die Schritte automatisieren (z. B. Husky/Git hooks) wenn du möchtest.---Siehe `scripts/hooks/pre-push.sample`.## Git Pre-Push Hook (Beispiel)- Wenn der Runner zu minimal ist (z. B. node fehlt), verwende das `full-20.04` image oder binde Host‑Ordner mit `-b` (`act -b`).- Bei `act`: nutze `-P ubuntu-latest=ghcr.io/catthehacker/ubuntu:full-20.04` und ggf. `--privileged` für tarpaulin/ASLR‑Operationen.- Wenn `llvm-profdata` auf Windows fehlschlägt, erwartet der Workflow eine ZIP‑Fallback‑Artefakt (`artifacts/coverage-fallback-*.zip`).## Troubleshooting- [ ] `cargo test` lokal durchlaufen- [ ] `cargo clippy --workspace --all-targets -- -D warnings` lokal durchlaufen- [ ] `act -j llvm-coverage` ausführen — Ergebnis: merged profdata & HTML generiert **oder** fallback ZIP (`artifacts/coverage-fallback-*.zip`) vorhanden- [ ] `scripts/run-local-ci.ps1` (oder `./scripts/run-local-ci.sh`) ausführen — Alle Schritte sollten grün sein## Checkliste (kurz)  - Du kannst `-j coverage` oder `-j llvm-coverage` einzeln ausführen, um Teilstücke zu prüfen.    ```    act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:full-20.04 -j llvm-coverage --privileged    ```bash  - Coverage job (benutze das volle image):- Mit `act` (lokales GH Actions emulation):  - Voller Lauf: `./scripts/run-local-ci.sh`- Linux/macOS (Bash):  - Empfohlen: `.\scripts\run-local-ci.ps1` (dies läuft Format, Clippy, Tests, UI/E2E falls vorhanden)unscripts\run-local-ci.ps1` je nach repo root)