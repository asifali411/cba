#requires -Version 5.1

[CmdletBinding()]
param(
    [string]$InstallDir = "$env:LOCALAPPDATA\cba\bin",
    [switch]$Force
)

$ErrorActionPreference = "Stop"
$BinName = "cba.exe"

function Write-Info { param([string]$Message) Write-Host "=> $Message" -ForegroundColor Cyan }
function Write-Warn { param([string]$Message) Write-Host "warning: $Message" -ForegroundColor Yellow }
function Write-ErrorAndExit {
    param([string]$Message)
    Write-Host "error: $Message" -ForegroundColor Red
    exit 1
}

# --------------------------------------------------------------------------
# Locate the installed binary
# --------------------------------------------------------------------------
$BinPath = Join-Path $InstallDir $BinName

if (-not (Test-Path $BinPath)) {
    # Fall back to whatever is resolvable on PATH.
    $onPath = Get-Command $BinName -ErrorAction SilentlyContinue
    if ($onPath) {
        $BinPath = $onPath.Source
        $InstallDir = Split-Path $BinPath -Parent
    } else {
        Write-ErrorAndExit "could not find '$BinName' in '$InstallDir' or on PATH. Use -InstallDir to specify its location."
    }
}

Write-Info "Found $BinName at: $BinPath"

# --------------------------------------------------------------------------
# Confirm and remove
# --------------------------------------------------------------------------
if (-not $Force) {
    $answer = Read-Host "Remove this file? [y/N]"
    if ($answer -notmatch '^[Yy]') {
        Write-Info "Aborted."
        exit 0
    }
}

try {
    Remove-Item -Path $BinPath -Force
} catch {
    Write-ErrorAndExit "failed to remove $BinPath ($($_.Exception.Message))"
}

Write-Info "Uninstalled $BinName from $BinPath"

# Remove the install directory too if it's now empty.
if ((Test-Path $InstallDir) -and -not (Get-ChildItem -Path $InstallDir -Force | Select-Object -First 1)) {
    Remove-Item -Path $InstallDir -Force -ErrorAction SilentlyContinue
    Write-Info "Removed empty directory $InstallDir"
}

# --------------------------------------------------------------------------
# Offer to clean up the user PATH entry
# --------------------------------------------------------------------------
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathEntries = $userPath -split ';' | Where-Object { $_ -ne "" }

if ($pathEntries -contains $InstallDir) {
    $removePath = $true
    if (-not $Force) {
        $answer = Read-Host "Remove '$InstallDir' from your user PATH? [Y/n]"
        $removePath = ($answer -eq "" -or $answer -match '^[Yy]')
    }

    if ($removePath) {
        $newEntries = $pathEntries | Where-Object { $_ -ne $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", ($newEntries -join ';'), "User")
        Write-Info "Removed $InstallDir from your user PATH. Restart your terminal for it to take effect everywhere."
    } else {
        Write-Warn "Left $InstallDir in your user PATH. Remove it manually if you no longer need it."
    }
}

Write-Info "Done."