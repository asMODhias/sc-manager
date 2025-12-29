# Runs adapter-gamelog unit tests
$ErrorActionPreference = 'Stop'
Push-Location -Path "${PSScriptRoot}\..\adapters\adapter-gamelog"
cargo test
Pop-Location