<#
.SYNOPSIS
    Remove ImageArtBorder from C:\Tools\ImageArtBorder and optional PATH entry.
#>
[CmdletBinding()]
param(
    [string] $InstallDir = "C:\Tools\ImageArtBorder"
)

$ErrorActionPreference = "Stop"

if (Test-Path -LiteralPath $InstallDir) {
    Remove-Item -LiteralPath $InstallDir -Recurse -Force
    Write-Host "Removed: $InstallDir"
} else {
    Write-Host "Not installed at: $InstallDir"
}

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -and $userPath -like "*$InstallDir*") {
    $parts = $userPath -split ";" | Where-Object { $_ -and $_ -ne $InstallDir }
    [Environment]::SetEnvironmentVariable("Path", ($parts -join ";"), "User")
    Write-Host "Removed from user PATH."
}
