# Build ImageArtBorder and assemble the distributable release folder.
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

Write-Host "=== Building release ===" -ForegroundColor Cyan
& "$Root\build.ps1"

$ExeSrc = Join-Path $Root "target\release\ImageArtBorder.exe"
if (-not (Test-Path $ExeSrc)) {
    throw "Missing $ExeSrc - build failed."
}

$OutDir = Join-Path $PSScriptRoot "ImageArtBorder"
if (Test-Path $OutDir) {
    Remove-Item $OutDir -Recurse -Force
}
New-Item -ItemType Directory -Path $OutDir | Out-Null

# Core binary and helpers
Copy-Item $ExeSrc (Join-Path $OutDir "ImageArtBorder.exe")
Copy-Item (Join-Path $PSScriptRoot "Add-Border.bat") $OutDir
Copy-Item (Join-Path $PSScriptRoot "Border-ExportedImages.ps1") $OutDir
Copy-Item (Join-Path $Root "LICENSE") $OutDir

# User documentation
$DocsOut = Join-Path $OutDir "docs"
New-Item -ItemType Directory -Path $DocsOut | Out-Null
Copy-Item (Join-Path $Root "docs\*.md") $DocsOut

# Short readme for the ZIP/folder root
Copy-Item (Join-Path $PSScriptRoot "README.txt") $OutDir

Write-Host ""
Write-Host "Release ready:" -ForegroundColor Green
Write-Host "  $OutDir"
Write-Host ""
Write-Host "Contents:"
Get-ChildItem $OutDir -Recurse | ForEach-Object {
    $rel = $_.FullName.Substring($OutDir.Length + 1)
    Write-Host "  $rel"
}
