# Local CI Validation Guide

Kurz: Diese Anleitung zeigt, wie Entwickler die lokale GitHub Actions Docker‑CI (via `act`) und die PowerShell/Unix‑Local CI Scripte ausführen, um sicherzustellen, dass PRs sauber sind bevor gepusht wird.

## Schnellbefehle

- Windows (PowerShell):
  - Voller Lauf: `.	ools\run-local-ci.ps1` (oder `.































Diese Datei ist zur schnellen Orientierung gedacht; ich habe die Coverage-Validierung auf `tarpaulin` umgestellt und die empfohlene lokale `act`-Ausführung ergänzt (siehe oben).

## Troubleshooting

- Wenn der Runner zu minimal ist (z. B. node fehlt), verwende das `full-20.04` image oder binde Host‑Ordner mit `-b` (`act -b`).
- Falls alte `.profraw`/`.info` Dateien vorhanden sind, lösche `profraw/*` und `artifacts/coverage/*` vor einem neuen Lauf.

### Tarpaulin / Coverage (lokal)

Tarpaulin benötigt ptrace/ASLR‑Kontrolle; auf Windows/Docker Desktop treten deshalb häufig Fehler wie `ASLR disable failed: EPERM` oder wiederholte `SIGTERM`/`SIGINT` auf. Verwende eine der folgenden Optionen, um lokal verlässliche Ergebnisse zu erzielen:

- Empfohlener (sicherer) Weg: Führe den Coverage‑Job **auf GitHub Actions (Linux runner)** aus (siehe Abschnitt "CI Alternative" weiter unten).

- Lokale Elevation (Docker): Starte den Container mit erweiterten Berechtigungen und deaktiviere ASLR innerhalb des Containers:

  - Docker (privilegierter Container):

    docker run --rm --privileged -v "${PWD}:/work" -w /work --entrypoint /bin/bash sc-manager/act-llvm:18.04 -c "echo 0 >/proc/sys/kernel/randomize_va_space || true; ./.github/scripts/run-tarpaulin-coverage-manual.sh"

  - Alternativ (wenn du keine --privileged Option möchtest):

    docker run --rm --cap-add=SYS_PTRACE --security-opt seccomp=unconfined -v "${PWD}:/work" -w /work sc-manager/act-llvm:18.04 /bin/bash -lc "echo 0 >/proc/sys/kernel/randomize_va_space || true; ./.github/scripts/run-tarpaulin-coverage-manual.sh"

  - Hinweise:
    - Wenn `echo 0 >/proc/sys/kernel/randomize_va_space` nicht funktioniert, setze den Container auf `--privileged` (nur für lokale Debugging‑Einsätze empfohlen).
    - Auf Windows kann es nötig sein, `docker run` **aus einem WSL2‑Shell** zu starten; WSL2 vermeidet einige ptrace/ASLR Beschränkungen des Windows‑Hosts.

- `act`-Beispiel (mit lokalem Image):

  act -P ubuntu-latest=sc-manager/act-llvm:18.04 -j tarpaulin-coverage --privileged

  Hinweis: `act` kann versuchen, Images zu pullen; stelle sicher, dass `sc-manager/act-llvm:18.04` lokal vorhanden ist (`docker images`).

### CI Alternative (empfohlen)

Führe den Tarpaulin‑Job als Teil der GitHub Actions auf einem Linux‑Runner aus. Die Runner erlauben ptrace/ASLR‑Deaktivierung und liefern verlässliche Artefakte. Beispiel‑Workflow: `/.github/workflows/coverage.yml` enthält einen `tarpaulin-coverage` Job, der LCOV und HTML‑Artefakte erzeugt.

### Fehlerdiagnose (häufige Fehlermeldungen)

- "ASLR disable failed: EPERM" → Starte Container mit `--privileged` oder `--cap-add=SYS_PTRACE --security-opt seccomp=unconfined` und deaktiviere ASLR im Container.
- Wiederholte `SIGTERM`/`SIGINT` → Container hat keine ausreichenden Rechte oder der Host killt Prozesse; versuche `--privileged` oder WSL2/GitHub Actions.
- `genhtml: command not found` → Stelle sicher, dass das Image `lcov` installiert hat (meistens Paketname `lcov`).

---


## Checkliste (kurz)

- [ ] `scripts/run-local-ci.ps1` (oder `./scripts/run-local-ci.sh`) ausführen — Alle Schritte sollten grün sein
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` lokal durchlaufen
- [ ] `cargo test` lokal durchlaufen
- [ ] `act -j tarpaulin-coverage` ausführen und prüfen, dass `artifacts/coverage/html` oder `artifacts/coverage/lcov` erzeugt wurde

---

Siehe auch: `scripts/hooks/pre-push.sample` für ein Beispiel‑Pre‑Push Hook.## Checkliste (kurz)  - Du kannst `-j coverage` oder `-j llvm-coverage` einzeln ausführen, um Teilstücke zu prüfen.    ```    act -P ubuntu-latest=ghcr.io/catthehacker/ubuntu:full-20.04 -j llvm-coverage --privileged    ```bash  - Coverage job (benutze das volle image):- Mit `act` (lokales GH Actions emulation):  - Voller Lauf: `./scripts/run-local-ci.sh`- Linux/macOS (Bash):  - Empfohlen: `.\scripts\run-local-ci.ps1` (dies läuft Format, Clippy, Tests, UI/E2E falls vorhanden)unscripts\run-local-ci.ps1` je nach repo root)