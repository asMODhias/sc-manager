📜 SC MANAGER V7 — MASTER SERVER & DISTRIBUTED MESH

Sections 1 – 4 (FINAL, BINDING)

Status: ABSOLUTE – ZERO DEVIATION
Paradigma: Local-First, Mesh-Driven, Audit-Safe
Copilot Rule: No questions, no reinterpretation

1️⃣ MASTER SERVER — AUTHOR AUTHORITY & AUDIT ANCHOR
1.1 Zweck (WHY)

Der Master Server ist kein Gameplay-Server und keine zentrale Datenbank, sondern:

Authoritative Trust Anchor

Unveränderbarer Audit-Notar

Legitimationsstelle für Updates & Plugins

Globale Konsistenz-Quelle (Hashes, nicht Inhalte)

Er ersetzt nicht das P2P-Mesh, sondern stabilisiert es.

1.2 Aufgaben (WHAT)

Der Master Server darf und muss:

Updates signieren (Genesis-Signatur)

Plugins registrieren & legitimieren

Audit-Events append-only speichern

DSGVO-konforme Hash-Identitäten verwalten

Geo-Policy & Legal Flags auswerten

Marketplace-Index veröffentlichen

Bug-, Crash-, ToS-Reports entgegennehmen

Der Master Server darf niemals:

Spielinhalte manipulieren

Live-State kontrollieren

Member- oder Org-Daten verändern

P2P-Traffic routen

Plugins ausführen

1.3 Datenmodell (IMMUTABLE)

Alle Daten sind append-only. Keine Deletes. Keine Updates.

AuditEvent {
event_id: SHA3-512
timestamp_utc
event_type
source_hash // User / Org / Node (anonym)
payload_hash // niemals Klartext
geo_region // ISO-3166-1
software_version
signature_author
}

➡ Rekonstruktion jederzeit möglich
➡ Manipulation mathematisch nachweisbar

1.4 Zugriff & Sicherheit

Author Access

Offline-Key (Hardware / Cold Storage)

mTLS + Key-Rotation

Nur für:

Update-Signierung

Plugin-Freigabe

Audit-Export

Client Access

Read-Only

Hash-Submission

Pull Marketplace Index

2️⃣ DISTRIBUTED P2P MESH — MINI-MASTER PRINZIP
2.1 Grundidee

Jede Installation ist ein Mini-Master.

Kein Client ist „dumm“.
Jeder validiert, cached, prüft.

2.2 Mesh-Eigenschaften

Transport: QUIC + mTLS

Discovery: DHT (Kademlia)

Sync: Gossip + Hash-Comparison

State: CRDTs (offline-fähig)

LocalNode
├─ validates updates
├─ verifies plugin signatures
├─ syncs org data
├─ reports hashes to master

2.3 Update-Flow (vereinfachtes Sequenzmodell)
Author → Master (sign update)
Master → Org-Leader Node (announce hash)
Org-Leader → P2P Mesh (delta chunks)
Members → Verify → Apply
Members → Report success hash → Master

➡ Kein zentraler Download
➡ Kein Single Point of Failure
➡ Bandbreiten-minimal

2.4 Ausfallverhalten

Master offline → Mesh funktioniert weiter

Author offline → keine neuen Updates, aber Betrieb stabil

Konflikte → Mehrheits-Hash gewinnt (Trust-Nodes)

3️⃣ PLUGIN MARKETPLACE — WASM-ONLY, FREE-ONLY
3.1 Grundregeln (NICHT VERHANDELBAR)

Kostenlos

Open Audit

WASM-Sandbox

TOS-Bound

Signaturpflicht

Kein Plugin darf:

Core ersetzen

TOS umgehen

Daten exfiltrieren

Netzwerkzugriff ohne Permission haben

3.2 Plugin-Lebenszyklus
Developer → Submit Plugin
Master → Static Scan + Policy Check
Author → Sign
Marketplace → Index Update
P2P → Distribution on Demand

3.3 Plugin-Typen
Typ Integration
Grinding Game.log (Read-Only)
Trading Externe APIs
RP Lore-konform
UI Panels / Overlays
Hardware StreamDeck / Razer
Media Twitch / YouTube
Language i18n Packs
Theme UI Skins
3.4 Plugin Security

WASM Runtime (Wasmtime)

Capability-based API

Runtime-Quotas (CPU, RAM, IO)

Kill-Switch via Master Policy

4️⃣ INSTALLER & DEPLOYMENT — TRI-HYBRID SYSTEM
4.1 Ein Installer. Drei Modi.

Setup.exe erkennt Umgebung automatisch.

Mode Zweck
GHOST Portable / USB
AGENT Gamer-PC
AUTHORITY Org-Server / Docker
4.2 Windows Integration (IDC-10)

Startmenü-Eintrag

Taskbar AppUserModelID

JumpLists (Check-In, Grinding)

Windows Service (SCM_Core_Svc)

Toast Notifications

Clean Uninstall (0 Reste)

4.3 Docker Mode (Authority)

Lokale Container (Postgres / Redis)

Keine Cloudpflicht

Audit-Export möglich

Headless Betrieb

4.4 Dependency Handling

Installer bietet Auswahl:

Bundled Runtime (Rust, VC++)

Docker Auto-Install (optional)

GPU-Acceleration optional

Offline-fähig

✅ ZWISCHENFAZIT (1–4)

✔ Master Server sinnvoll & notwendig
✔ Kein Bruch mit V6/V6.5
✔ Rechtlich sauber
✔ Technisch realistisch
✔ Copilot kann direkt bauen
