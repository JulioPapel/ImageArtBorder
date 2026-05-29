# Build Linux x64 release via Docker build context (works when WSL/bind mounts fail).
# Author: Júlio Papel <info@juliopapel.pt>
$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent
Set-Location $Root

$image = "imageartborder-linux-build"
Write-Host "=== Docker build: Linux x64 ===" -ForegroundColor Cyan
docker build -f "$Root\scripts\Dockerfile.linux-build" -t $image $Root
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

$version = (Select-String -Path "$Root\Cargo.toml" -Pattern '^version = "(.+)"').Matches.Groups[1].Value
$distName = "ImageArtBorder-$version-linux-x64"
$container = "imageartborder-linux-extract"

cmd /c "docker rm -f $container 2>nul" | Out-Null
$cid = docker create --name $container $image 2>&1
if ($LASTEXITCODE -ne 0) { throw "docker create failed: $cid" }
$hostDist = "$Root\release\dist\$distName"
if (Test-Path $hostDist) { Remove-Item -Recurse -Force $hostDist }
New-Item -ItemType Directory -Force -Path "$Root\release\dist" | Out-Null
docker cp "${container}:/build/release/dist/$distName" "$Root\release\dist\"
if ($LASTEXITCODE -ne 0) { throw "docker cp failed" }
cmd /c "docker rm -f $container 2>nul" | Out-Null

Write-Host "Linux release: $hostDist" -ForegroundColor Green
