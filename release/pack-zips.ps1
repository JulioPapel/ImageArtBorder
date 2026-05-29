# Zip release/dist folders for GitHub release attachments.
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
$Dist = Join-Path $PSScriptRoot "dist"
$Zips = Join-Path $PSScriptRoot "zips"

if (-not (Test-Path $Dist)) {
    throw "No release/dist folder. Run pack-release.ps1 and/or build-linux-docker.ps1 first."
}

if (Test-Path $Zips) { Remove-Item $Zips -Recurse -Force }
New-Item -ItemType Directory -Path $Zips -Force | Out-Null

Get-ChildItem $Dist -Directory | ForEach-Object {
    $zipPath = Join-Path $Zips "$($_.Name).zip"
    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
    Compress-Archive -Path $_.FullName -DestinationPath $zipPath -CompressionLevel Optimal
    $sizeMb = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)
    Write-Host "  $($_.Name).zip ($sizeMb MB)" -ForegroundColor Green
}

Write-Host ""
Write-Host "Attach these to your GitHub release:" -ForegroundColor Cyan
Write-Host "  $Zips"
