# Build Windows release and Linux/macOS via Docker (WSL only if /mnt/c works).
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

function Test-WslCanAccessRepo {
    param([string]$RepoRoot)
    $prev = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $wslPath = (wsl wslpath -a $RepoRoot 2>$null | Out-String).Trim()
        if (-not $wslPath) { return $false }
        wsl bash -lc "test -d '$wslPath' && test -f '$wslPath/Cargo.toml'" 2>$null | Out-Null
        return ($LASTEXITCODE -eq 0)
    } finally {
        $ErrorActionPreference = $prev
    }
}

function Invoke-LinuxViaWsl {
    param([string]$RepoRoot)
    $prev = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        $wslPath = (wsl wslpath -a $RepoRoot 2>$null | Out-String).Trim()
        if (-not $wslPath) {
            $wslPath = ($RepoRoot -replace '\\', '/')
        }
        wsl bash -lc "cd '$wslPath' && chmod +x scripts/*.sh release/*.sh && ./scripts/build-linux.sh" 2>&1 | ForEach-Object { Write-Host $_ }
        return ($LASTEXITCODE -eq 0)
    } finally {
        $ErrorActionPreference = $prev
    }
}

Write-Host "=== ImageArtBorder multi-platform build ===" -ForegroundColor Cyan

& "$Root\release\pack-release.ps1"

$linuxOk = $false
$wsl = Get-Command wsl -ErrorAction SilentlyContinue
if ($wsl -and (Test-WslCanAccessRepo -RepoRoot $Root)) {
    Write-Host ""
    Write-Host "=== Linux x64 (WSL) ===" -ForegroundColor Cyan
    $linuxOk = Invoke-LinuxViaWsl -RepoRoot $Root
    if (-not $linuxOk) {
        Write-Host "WSL Linux build failed; will try Docker." -ForegroundColor Yellow
    }
} elseif ($wsl) {
    Write-Host ""
    Write-Host "WSL cannot access this folder (common on some setups). Skipping WSL." -ForegroundColor Yellow
}

if (-not $linuxOk) {
    $docker = Get-Command docker -ErrorAction SilentlyContinue
    if ($docker) {
        Write-Host ""
        Write-Host "=== Linux x64 (Docker) ===" -ForegroundColor Cyan
        & "$Root\scripts\build-linux-docker.ps1"
        if ($LASTEXITCODE -eq 0) { $linuxOk = $true }
    } else {
        Write-Host ""
        Write-Host "Linux build skipped (WSL/Docker unavailable)." -ForegroundColor Yellow
        Write-Host "  .\scripts\build-linux-docker.ps1"
    }
}

$hasMac = @(Get-ChildItem "$Root\release\dist" -Directory -Filter "ImageArtBorder-*-macos-arm64" -ErrorAction SilentlyContinue).Count -gt 0
if (-not $hasMac) {
    $docker = Get-Command docker -ErrorAction SilentlyContinue
    if ($docker) {
        Write-Host ""
        Write-Host "=== macOS (arm64 + x64, Docker) ===" -ForegroundColor Cyan
        & "$Root\scripts\build-macos-docker.ps1"
    } else {
        Write-Host ""
        Write-Host "macOS: .\scripts\build-macos-docker.ps1  or  ./scripts/build-macos.sh on a Mac" -ForegroundColor Yellow
    }
} else {
    Write-Host ""
    Write-Host "macOS packages already present in release\dist\ (skipping Docker macOS build)." -ForegroundColor DarkGray
}

Write-Host ""
Write-Host "Zip for GitHub: .\release\pack-zips.bat" -ForegroundColor Cyan

Write-Host ""
Write-Host "Artifacts under release\dist\" -ForegroundColor Green
if (Test-Path "$Root\release\dist") {
    Get-ChildItem "$Root\release\dist" -Directory | ForEach-Object { Write-Host "  $($_.Name)" }
}
