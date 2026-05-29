<#
.SYNOPSIS
    Install ImageArtBorder to C:\Tools\ImageArtBorder (or a custom folder).

.DESCRIPTION
    Author: Júlio Papel <info@juliopapel.pt>
    License: MIT

    Copies the release package (exe, scripts, docs) to the install directory.
    Run from the extracted ZIP, from release\ImageArtBorder after pack-release.ps1,
    or from the repository after building.

.PARAMETER InstallDir
    Target directory (default: C:\Tools\ImageArtBorder).

.PARAMETER SourceDir
    Folder containing ImageArtBorder.exe (default: auto-detected).

.PARAMETER AddToUserPath
    Append InstallDir to the current user's PATH.
#>
[CmdletBinding()]
param(
    [string] $InstallDir = "C:\Tools\ImageArtBorder",
    [string] $SourceDir = "",
    [switch] $AddToUserPath
)

$ErrorActionPreference = "Stop"

function Resolve-SourceDir {
    param([string] $Explicit)
    if ($Explicit) {
        return (Resolve-Path -LiteralPath $Explicit).Path
    }
    if (Test-Path -LiteralPath (Join-Path $PSScriptRoot "ImageArtBorder.exe")) {
        return $PSScriptRoot
    }
    $nested = Join-Path $PSScriptRoot "ImageArtBorder"
    if (Test-Path -LiteralPath (Join-Path $nested "ImageArtBorder.exe")) {
        return (Resolve-Path -LiteralPath $nested).Path
    }
    throw "Cannot find ImageArtBorder.exe. Run pack-release.ps1 first or set -SourceDir."
}

$src = Resolve-SourceDir -Explicit $SourceDir
$versionFile = Join-Path $src "VERSION"
$version = if (Test-Path $versionFile) { Get-Content $versionFile -Raw } else { "unknown" }

Write-Host "ImageArtBorder installer" -ForegroundColor Cyan
Write-Host "  Version:  $($version.Trim())"
Write-Host "  From:     $src"
Write-Host "  To:       $InstallDir"
Write-Host ""

if (Test-Path $InstallDir) {
    Write-Host "Updating existing installation..."
    Remove-Item -LiteralPath $InstallDir -Recurse -Force
}
New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null

Copy-Item -Path (Join-Path $src "*") -Destination $InstallDir -Recurse -Force

Get-ChildItem -LiteralPath $InstallDir -Recurse -File | Where-Object {
    $_.Extension -in ".exe", ".ps1", ".bat"
} | ForEach-Object {
    if ($_.Extension -eq ".exe") {
        Unblock-File -LiteralPath $_.FullName -ErrorAction SilentlyContinue
    }
}

if ($AddToUserPath) {
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($userPath -notlike "*$InstallDir*") {
        $newPath = if ($userPath) { "$userPath;$InstallDir" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = "$env:Path;$InstallDir"
        Write-Host "Added to user PATH: $InstallDir"
    }
}

Write-Host ""
Write-Host "Installed successfully." -ForegroundColor Green
Write-Host "  $InstallDir\ImageArtBorder.exe"
Write-Host ""
Write-Host "Verify:"
Write-Host "  & `"$InstallDir\ImageArtBorder.exe`" --version"
Write-Host "  & `"$InstallDir\ImageArtBorder.exe`" --help"
