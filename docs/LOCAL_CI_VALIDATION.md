# Local CI Validation Guide

Kurz: Diese Anleitung zeigt, wie Entwickler die lokale GitHub Actions Docker‑CI (via `act`) und die PowerShell/Unix‑Local CI Scripte ausführen, um sicherzustellen, dass PRs sauber sind bevor gepusht wird.

## Schnellbefehle

- Windows (PowerShell):
  - Voller Lauf: `.	ools\run-local-ci.ps1` (oder `.































Diese Datei ist zur schnellen Orientierung gedacht; ich habe die Coverage-Validierung auf `tarpaulin` umgestellt und die empfohlene lokale `act`-Ausführung ergänzt (siehe oben).

## Troubleshooting

- Wenn der Runner zu minimal ist (z. B. node fehlt), verwende das `full-20.04` image oder binde Host‑Ordner mit `-b` (`act -b`).
- Bei `act`: nutze `-P ubuntu-latest=sc-manager/act-llvm:18.04 -j tarpaulin-coverage --privileged` für lokale Coverage‑Validierung.
- Falls alte `.profraw`/`.info` Dateien vorhanden sind, lösche `profraw/*` und `artifacts/coverage/*` vor einem neuen Lauf.

## Checkliste (kurz)

- [ ] `scripts/run-local-ci.ps1` (oder `./scripts/run-local-ci.sh`) ausführen — Alle Schritte sollten grün sein
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` lokal durchlaufen
- [ ] `cargo test` lokal durchlaufen
- [ ] `act -j tarpaulin-coverage` ausführen und prüfen, dass `artifacts/coverage/html` oder `artifacts/coverage/lcov` erzeugt wurde

---

Siehe auch: `scripts/hooks/pre-push.sample` für ein Beispiel‑Pre‑Push Hook.## Checkliste (kurz)  - Du kannst `-j coverage` oder `-j llvm-coverage` einzeln ausführen, um Teilstücke zu prüfen.    ```    act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:full-20.04 -j llvm-coverage --privileged    ```bash  - Coverage job (benutze das volle image):- Mit `act` (lokales GH Actions emulation):  - Voller Lauf: `./scripts/run-local-ci.sh`- Linux/macOS (Bash):  - Empfohlen: `.\scripts\run-local-ci.ps1` (dies läuft Format, Clippy, Tests, UI/E2E falls vorhanden)unscripts\run-local-ci.ps1` je nach repo root)