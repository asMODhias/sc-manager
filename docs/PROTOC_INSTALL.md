# protoc (Protocol Buffers compiler) — Install guide

Some crates in this workspace (notably `libp2p`/`prost`) require the `protoc` binary at build time. This document explains how to install it locally and how the repository's helper scripts can optionally install it for you.

## Quick manual install

- macOS (Homebrew):
  - brew install protobuf
- Debian/Ubuntu:
  - sudo apt-get update && sudo apt-get install -y protobuf-compiler
- RHEL/CentOS:
  - sudo yum install -y protobuf-compiler
- Arch Linux:
  - sudo pacman -S protobuf
- Windows:
  - choco install protoc
  - or scoop install protoc

## Automated install via helper scripts

The repository includes cross-platform helper scripts:

- `scripts/install-protoc.sh` — Linux/macOS helper (downloads prebuilt release when packages aren't available)
- `scripts/install-protoc.ps1` — PowerShell helper (Windows, macOS, Linux)

Usage examples:

- Bash (attempt install):
  PROTOC_VERSION=23.3 ./scripts/install-protoc.sh

- PowerShell (attempt install):
  $env:PROTOC_VERSION = '23.3'; .\\scripts\\install-protoc.ps1

## Local CI integration

If `protoc` is not present, the local CI (`scripts/run-local-ci.sh` / `scripts/run-local-ci.ps1`) will warn and skip crates that require `protoc` by default. To ask the CI to try installing `protoc` automatically, set the environment variable `PROTOC_AUTO_INSTALL=1` before running the local CI script. Example:

- Bash:
  PROTOC_AUTO_INSTALL=1 ./scripts/run-local-ci.sh

- PowerShell:
  $env:PROTOC_AUTO_INSTALL = '1'; .\\scripts\\run-local-ci.ps1

If automatic installation fails, follow the manual install instructions above and re-run the CI.
