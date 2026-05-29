# Pack macOS release folder(s) after cross-build (Windows host).
# Usage: .\scripts\pack-macos.ps1 macos-arm64 target\aarch64-apple-darwin\release\ImageArtBorder
param(
    [Parameter(Mandatory = $true)][string]$Platform,
    [Parameter(Mandatory = $true)][string]$BinaryPath
)

$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
$version = (Select-String -Path "$Root\Cargo.toml" -Pattern '^version = "(.+)"').Matches.Groups[1].Value
$OutDir = Join-Path $Root "release\dist\ImageArtBorder-${version}-${Platform}"

if (-not (Test-Path $BinaryPath)) { throw "Binary not found: $BinaryPath" }

if (Test-Path $OutDir) { Remove-Item $OutDir -Recurse -Force }
New-Item -ItemType Directory -Path "$OutDir\docs" -Force | Out-Null

Copy-Item $BinaryPath (Join-Path $OutDir "ImageArtBorder")
Set-Content -Path (Join-Path $OutDir "VERSION") -Value $version -NoNewline
Copy-Item "$Root\LICENSE" $OutDir
Copy-Item "$Root\release\README-unix.txt" (Join-Path $OutDir "README.txt")
Copy-Item "$Root\docs\*.md" (Join-Path $OutDir "docs")
Copy-Item "$Root\release\install.sh", "$Root\release\uninstall.sh", "$Root\release\add-border.sh", "$Root\release\border-exported-images.sh" $OutDir

Write-Host "Release ready: $OutDir" -ForegroundColor Green
