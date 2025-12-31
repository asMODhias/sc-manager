<#
PowerShell helper to install protoc on Windows / macOS / Linux (where possible)
Usage: $env:PROTOC_VERSION = '23.3'; .\scripts\install-protoc.ps1
Set $env:INSTALL_PREFIX to change installation directory (default: $HOME\bin)
#>
[CmdletBinding()]
param()

$protocVersion = $env:PROTOC_VERSION -or '23.3'
$installPrefix = $env:INSTALL_PREFIX -or "$env:USERPROFILE\bin"
if (-not (Test-Path $installPrefix)) { New-Item -ItemType Directory -Path $installPrefix | Out-Null }

Write-Host "[install-protoc] installing protoc $protocVersion to $installPrefix"

function Found-Protoc { if ($env:PROTOC) { return (Test-Path $env:PROTOC) } ; return (Get-Command protoc -ErrorAction SilentlyContinue) }

# Try platform-native installers
if ($IsWindows) {
    # Try choco
    if (Get-Command choco -ErrorAction SilentlyContinue) {
        Write-Host "[install-protoc] choco detected — trying 'choco install protoc'"
        try {
            & choco install protoc -y
        } catch {
            Write-Host "[install-protoc] choco install failed:" $_ -ForegroundColor Yellow
        }
        if (Found-Protoc) { Write-Host "[install-protoc] protoc installed via choco" ; exit 0 }
    }
    # Try scoop
    if (Get-Command scoop -ErrorAction SilentlyContinue) {
        Write-Host "[install-protoc] scoop detected — trying 'scoop install protoc'"
        try {
            & scoop install protoc
        } catch {
            Write-Host "[install-protoc] scoop install failed:" $_ -ForegroundColor Yellow
        }
        if (Found-Protoc) { Write-Host "[install-protoc] protoc installed via scoop" ; exit 0 }
    }
    # Fallback: download zip release
    $url = "https://github.com/protocolbuffers/protobuf/releases/download/v$protocVersion/protoc-$protocVersion-win64.zip"
    $tmp = Join-Path $env:TEMP "protoc-$protocVersion.zip"
    Write-Host "[install-protoc] downloading $url"
    Invoke-WebRequest -Uri $url -OutFile $tmp -UseBasicParsing
    $extractPath = Join-Path $env:TEMP "protoc-extract"
    Remove-Item -Recurse -Force $extractPath -ErrorAction SilentlyContinue
    Expand-Archive -Path $tmp -DestinationPath $extractPath -Force
    $protocBin = Join-Path $extractPath 'bin\protoc.exe'
    if (Test-Path $protocBin) {
        Copy-Item $protocBin -Destination (Join-Path $installPrefix 'protoc.exe') -Force
        Write-Host "[install-protoc] protoc installed to $installPrefix\protoc.exe"
        Write-Host "[install-protoc] add $installPrefix to PATH or set PROTOC env var to the binary path."
        exit 0
    }
    Write-Host "[install-protoc] download/install failed" -ForegroundColor Red
    exit 1
} else {
    # macOS / Linux
    if (Get-Command brew -ErrorAction SilentlyContinue) {
        Write-Host "[install-protoc] Homebrew detected — attempting 'brew install protobuf'"
        try {
            & brew install protobuf
        } catch {
            Write-Host "[install-protoc] brew install failed: $_" -ForegroundColor Yellow
        }
        if (Found-Protoc) { Write-Host "[install-protoc] protoc installed via brew" ; exit 0 }
    }

    if (Get-Command apt-get -ErrorAction SilentlyContinue) {
        Write-Host "[install-protoc] apt-get detected — attempting non-interactive apt-get install"
        $sudoAvailable = $false
        try { sudo -n true; $sudoAvailable = $true } catch { $sudoAvailable = $false }
        if ($sudoAvailable) {
            try {
                sudo apt-get update; sudo apt-get install -y protobuf-compiler
            } catch {
                Write-Host "[install-protoc] apt-get failed:" $_ -ForegroundColor Yellow
            }
        } elseif ([int](whoami -u) -eq 0) {
            try {
                apt-get update; apt-get install -y protobuf-compiler
            } catch {
                Write-Host "[install-protoc] apt-get failed:" $_ -ForegroundColor Yellow
            }
        } else {
            Write-Host "[install-protoc] sudo not available or requires password; skipping apt-get (will try download fallback)" -ForegroundColor Yellow
        }
        if (Found-Protoc) { Write-Host "[install-protoc] protoc installed via apt-get" ; exit 0 }
    }

    if (Get-Command yum -ErrorAction SilentlyContinue) {
        Write-Host "[install-protoc] yum detected — attempting non-interactive yum install"
        $sudoAvailable = $false
        try { sudo -n true; $sudoAvailable = $true } catch { $sudoAvailable = $false }
        if ($sudoAvailable) {
            try {
                sudo yum install -y protobuf-compiler
            } catch {
                Write-Host "[install-protoc] yum failed: $_" -ForegroundColor Yellow
            }
        } elseif ([int](whoami -u) -eq 0) {
            try {
                yum install -y protobuf-compiler
            } catch {
                Write-Host "[install-protoc] yum failed: $_" -ForegroundColor Yellow
            }
        } else {
            Write-Host "[install-protoc] sudo not available or requires password; skipping yum (will try download fallback)" -ForegroundColor Yellow
        }
        if (Found-Protoc) { Write-Host "[install-protoc] protoc installed via yum" ; exit 0 }
    }

    # Fallback to download
    $os = if ($IsMacOS) { 'osx' } else { 'linux' }
    $url = "https://github.com/protocolbuffers/protobuf/releases/download/v$protocVersion/protoc-$protocVersion-$os-x86_64.zip"
    $tmp = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "protoc-$protocVersion.zip")
    Write-Host "[install-protoc] downloading $url"
    Invoke-WebRequest -Uri $url -OutFile $tmp -UseBasicParsing
    $extractPath = [System.IO.Path]::Combine([System.IO.Path]::GetTempPath(), "protoc-extract")
    Remove-Item -Recurse -Force $extractPath -ErrorAction SilentlyContinue
    Expand-Archive -Path $tmp -DestinationPath $extractPath -Force
    $protocSrc = Join-Path $extractPath 'bin/protoc'
    if (Test-Path $protocSrc) {
        Copy-Item $protocSrc -Destination (Join-Path $installPrefix 'protoc') -Force
        icacls (Join-Path $installPrefix 'protoc') /grant Everyone:RX | Out-Null
        Write-Host "[install-protoc] protoc installed to $installPrefix/protoc"
        Write-Host "[install-protoc] ensure $installPrefix is in your PATH or set PROTOC env var to the binary path."
        exit 0
    }
    Write-Host "[install-protoc] download/install failed" -ForegroundColor Red
    exit 1
}