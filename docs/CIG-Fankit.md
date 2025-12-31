# CIG Fankit Integration

Kurz: Dieses Dokument beschreibt den Umgang mit dem von Cloud Imperium Games (CIG) bereitgestellten Fankit, die Lizenzanforderungen sowie den Update-Path für dieses Repo.

## Lizenz und TOS ✅
- Bevor Assets importiert werden, MUSS die mitgelieferte LICENSE/TOS-Datei geprüft werden.
- Standardprozess: das Importskript kopiert die Lizenz in `assets/fankit/LICENSE_CIG_FANKIT.txt`.
- Falls die Lizenz nicht das direkte Re-Commit von Assets erlaubt, darf der Import nicht erfolgen. Bitte Lizenz lesen und gegebenenfalls Rechtsrat einholen.

## Speicherort
- Alle zulässigen Assets werden in `assets/fankit/` abgelegt.
- Die Struktur wird beibehalten und ein Manifest `assets/fankit/fankit-manifest.json` wird angelegt, das Pfad, Größe, SHA256-Hash, Quelle und Importzeit dokumentiert.

## Import / Update Ablauf 🔁
1. Lege den Fankit-Ordner lokal bereit (z. B. `D:\Old Data\Downloads\Star Citizen Fankit_...`).
2. Ausführen (Linux/macOS):
   ```bash
   ./scripts/import-fankit.sh /path/to/Fankit_Source
   ```
   oder PowerShell (Windows):
   ```powershell
   ./scripts/import-fankit.ps1 -SourcePath "C:\Path\To\Fankit"
   ```
3. Das Skript prüft auf eine LICENSE/TOS-Datei, kopiert erlaubte Dateien, erzeugt `fankit-manifest.json` und kopiert die Lizenz nach `assets/fankit/LICENSE_CIG_FANKIT.txt`.
4. Prüfen: `scripts/check-fankit-manifest.sh` ausführen (oder CI wird das prüfen).
5. Commit & PR: `git add assets/fankit && git commit -m "Add CIG Fankit assets (import)"`.

## CI & Checks
- Wir bieten ein Hilfs-Skript `scripts/check-fankit-manifest.sh` das: license presence, dateiexistenz und SHA256-validierung prüft.
- In CI: `scripts/check-fankit-manifest.sh` sollte ausgeführt werden, sobald `assets/fankit` verändert wird.

## Verwendung in Projekt
- UI-Komponenten oder Dokumentationsbeispiele dürfen die Dateien referenzieren (z. B. `assets/fankit/logo.png`).
- Achte darauf, dass alle Verwendungen der Lizenz entsprechen (z. B. Attribution, keine kommerzielle Verwendung falls untersagt).

## Changelog & Updates
- Jeder Import erstellt automatisch einen Eintrag am Ende dieses Dokuments (Datum, Quelle, Anzahl importierter Dateien).

---
*Hinweis: Dieses Repo enthält nur Assets, die laut der jeweiligen TOS importierbar sind. Füge keine Dateien hinzu, ohne die Lizenzbedingungen zu prüfen.*


## Manifest generated: 2025-12-30T20:39:46.4345916+01:00
- Quelle: local-import (added directly to repo under assets/fankit)
- Dateianzahl aufgeführt im Manifest: 231
- Lizenz: D:\Data\Star Citizen\Tools\Manager\assets\fankit\LICENSE_CIG_FANKIT.txt

