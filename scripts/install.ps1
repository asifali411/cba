#requires -Version 5.1

[CmdletBinding()]
param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\cba\bin",
    [string]$Repo = "asifali411/cba"
)

$ErrorActionPreference = "Stop"
$BinName = "cba.exe"

function Write-Info  { param([string]$Message) Write-Host "=> $Message" -ForegroundColor Cyan }
function Write-Warn  { param([string]$Message) Write-Host "warning: $Message" -ForegroundColor Yellow }
function Write-ErrorAndExit {
    param([string]$Message)
    Write-Host "error: $Message" -ForegroundColor Red
    exit 1
}

# --------------------------------------------------------------------------
# Detect architecture (matrix currently publishes windows-x86_64 only)
# --------------------------------------------------------------------------
$archRaw = $env:PROCESSOR_ARCHITECTURE
switch -Regex ($archRaw) {
    "AMD64" { $ArchTag = "x86_64" }
    "ARM64" { Write-ErrorAndExit "no windows-aarch64 release is currently published; only windows-x86_64 is available" }
    default { Write-ErrorAndExit "unsupported architecture: $archRaw" }
}

$TargetName  = "windows-$ArchTag"
$Archive     = "cba-$TargetName.zip"
$ChecksumFile = "$Archive.sha256"

Write-Info "Detected platform: $TargetName"

# --------------------------------------------------------------------------
# Resolve download URLs
# --------------------------------------------------------------------------
if ($Version -eq "latest") {
    $BaseUrl = "https://github.com/$Repo/releases/latest/download"
    Write-Info "Installing latest release of $Repo"
} else {
    $BaseUrl = "https://github.com/$Repo/releases/download/$Version"
    Write-Info "Installing $Repo $Version"
}

$ArchiveUrl  = "$BaseUrl/$Archive"
$ChecksumUrl = "$BaseUrl/$ChecksumFile"

# --------------------------------------------------------------------------
# Download, verify, extract, install
# --------------------------------------------------------------------------
$WorkDir = Join-Path ([System.IO.Path]::GetTempPath()) ([System.IO.Path]::GetRandomFileName())
New-Item -ItemType Directory -Path $WorkDir | Out-Null

try {
    $ArchivePath  = Join-Path $WorkDir $Archive
    $ChecksumPath = Join-Path $WorkDir $ChecksumFile

    Write-Info "Downloading $ArchiveUrl"
    try {
        Invoke-WebRequest -Uri $ArchiveUrl -OutFile $ArchivePath -UseBasicParsing
    } catch {
        Write-ErrorAndExit "failed to download archive. Does release '$Version' contain an asset for $TargetName? ($($_.Exception.Message))"
    }

    Write-Info "Downloading checksum"
    $checksumOk = $true
    try {
        Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumPath -UseBasicParsing
    } catch {
        $checksumOk = $false
        Write-Warn "checksum file not found, skipping verification"
    }

    if ($checksumOk) {
        Write-Info "Verifying checksum"
        $expectedLine = (Get-Content $ChecksumPath -Raw).Trim()
        $expected = ($expectedLine -split '\s+')[0].ToLower()
        $actual = (Get-FileHash -Path $ArchivePath -Algorithm SHA256).Hash.ToLower()
        if ($expected -ne $actual) {
            Write-ErrorAndExit "checksum verification failed (expected $expected, got $actual)"
        }
        Write-Info "Checksum OK"
    }

    Write-Info "Extracting archive"
    $ExtractDir = Join-Path $WorkDir "extracted"
    Expand-Archive -Path $ArchivePath -DestinationPath $ExtractDir -Force

    $BinSrc = Join-Path $ExtractDir $BinName
    if (-not (Test-Path $BinSrc)) {
        # fall back to a recursive search in case the archive nests the exe
        $found = Get-ChildItem -Path $ExtractDir -Filter $BinName -Recurse | Select-Object -First 1
        if (-not $found) {
            Write-ErrorAndExit "binary '$BinName' not found in archive"
        }
        $BinSrc = $found.FullName
    }

    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    $BinDest = Join-Path $InstallDir $BinName
    Copy-Item -Path $BinSrc -Destination $BinDest -Force

    Write-Info "Installed $BinName to $BinDest"
}
finally {
    Remove-Item -Path $WorkDir -Recurse -Force -ErrorAction SilentlyContinue
}

# --------------------------------------------------------------------------
# PATH check (user-level)
# --------------------------------------------------------------------------
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathEntries = $userPath -split ';' | Where-Object { $_ -ne "" }

if ($pathEntries -notcontains $InstallDir) {
    Write-Warn "$InstallDir is not in your PATH."
    $answer = Read-Host "Add it to your user PATH now? [Y/n]"
    if ($answer -eq "" -or $answer -match '^[Yy]') {
        $newPath = if ($userPath) { "$userPath;$InstallDir" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Info "Added $InstallDir to your user PATH. Restart your terminal for it to take effect everywhere."
    } else {
        Write-Warn "Skipped. Add this manually if needed:"
        Write-Warn "  [Environment]::SetEnvironmentVariable('Path', `"`$env:Path;$InstallDir`", 'User')"
    }
} else {
    $env:Path = "$env:Path;$InstallDir"
}

Write-Info "Done. Run '$BinName --version' to verify (open a new terminal if the command isn't found)."