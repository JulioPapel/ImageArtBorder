# Build ImageArtBorder and assemble the Windows x64 release folder.
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

$version = (Select-String -Path (Join-Path $Root "Cargo.toml") -Pattern '^version = "(.+)"' |
    ForEach-Object { $_.Matches.Groups[1].Value })

Write-Host "=== Building ImageArtBorder v$version (windows-x64) ===" -ForegroundColor Cyan
& "$Root\build.ps1"

$ExeSrc = Join-Path $Root "target\release\ImageArtBorder.exe"
if (-not (Test-Path $ExeSrc)) {
    throw "Missing $ExeSrc - build failed."
}

$DistRoot = Join-Path $PSScriptRoot "dist"
$OutDir = Join-Path $DistRoot "ImageArtBorder-${version}-windows-x64"
if (Test-Path $OutDir) {
    Remove-Item $OutDir -Recurse -Force
}
New-Item -ItemType Directory -Path $OutDir -Force | Out-Null

Set-Content -Path (Join-Path $OutDir "VERSION") -Value $version -NoNewline

Copy-Item $ExeSrc (Join-Path $OutDir "ImageArtBorder.exe")
Copy-Item (Join-Path $PSScriptRoot "Add-Border.bat") $OutDir
Copy-Item (Join-Path $PSScriptRoot "Border-ExportedImages.ps1") $OutDir
Copy-Item (Join-Path $PSScriptRoot "install.ps1") $OutDir
Copy-Item (Join-Path $PSScriptRoot "uninstall.ps1") $OutDir
Copy-Item (Join-Path $Root "LICENSE") $OutDir

$DocsOut = Join-Path $OutDir "docs"
New-Item -ItemType Directory -Path $DocsOut | Out-Null
Copy-Item (Join-Path $Root "docs\*.md") $DocsOut

Copy-Item (Join-Path $PSScriptRoot "README.txt") $OutDir

# Legacy symlink-style folder for existing Windows install docs
$LegacyDir = Join-Path $PSScriptRoot "ImageArtBorder"
if (Test-Path $LegacyDir) { Remove-Item $LegacyDir -Recurse -Force }
Copy-Item $OutDir $LegacyDir -Recurse

Write-Host ""
Write-Host "Release v$version ready:" -ForegroundColor Green
Write-Host "  $OutDir"
Write-Host ""
Write-Host "Install to C:\Tools\ImageArtBorder:"
Write-Host "  powershell -ExecutionPolicy Bypass -File `"$OutDir\install.ps1`""
