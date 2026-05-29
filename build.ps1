# Build ImageArtBorder (release, Windows x64).
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot

$env:CARGO_HTTP_CHECK_REVOCATION = "false"

Write-Host "Building ImageArtBorder (release)..."
cargo build --release

$exe = Join-Path $PSScriptRoot "target\release\ImageArtBorder.exe"
if (-not (Test-Path $exe)) {
    throw "Build failed: $exe not found"
}

Write-Host "Built: $exe"
& $exe --help
