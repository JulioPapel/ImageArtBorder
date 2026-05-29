# Build macOS arm64 + x64 release packages via Docker + cargo-zigbuild.
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

$image = "imageartborder-macos-build"
Write-Host "=== Docker build: macOS (arm64 + x64) ===" -ForegroundColor Cyan
docker build -f "$Root\scripts\Dockerfile.macos-build" -t $image $Root
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$version = (Select-String -Path "$Root\Cargo.toml" -Pattern '^version = "(.+)"').Matches.Groups[1].Value
$platforms = @("macos-arm64", "macos-x64")
$container = "imageartborder-macos-extract"

cmd /c "docker rm -f $container 2>nul" | Out-Null
$cid = docker create --name $container $image 2>&1
if ($LASTEXITCODE -ne 0) { throw "docker create failed: $cid" }

New-Item -ItemType Directory -Force -Path "$Root\release\dist" | Out-Null
foreach ($p in $platforms) {
    $distName = "ImageArtBorder-${version}-${p}"
    $hostDist = "$Root\release\dist\$distName"
    if (Test-Path $hostDist) { Remove-Item $hostDist -Recurse -Force }
    docker cp "${container}:/build/release/dist/$distName" "$Root\release\dist\"
    if ($LASTEXITCODE -ne 0) { throw "docker cp failed for $distName" }
    Write-Host "  $hostDist" -ForegroundColor Green
}
cmd /c "docker rm -f $container 2>nul" | Out-Null

Write-Host "Done. Run .\release\pack-zips.ps1 to create GitHub zips." -ForegroundColor Cyan
