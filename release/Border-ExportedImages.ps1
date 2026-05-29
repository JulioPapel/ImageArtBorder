<#
.SYNOPSIS
    Apply ImageArtBorder to every image in a Capture One (or other) export folder.

.DESCRIPTION
    Author: Júlio Papel <info@juliopapel.pt>
    License: MIT

    Run this after exporting from Capture One so all JPEG/PNG/TIFF files in the
    folder receive the same border. See docs\capture-one.md in this package.

.PARAMETER Folder
    Directory containing exported images.

.PARAMETER Border
    Diagonal percent increase, same as -b (default 6.0).

.PARAMETER Color
    Border color as #RRGGBB (default #FFFFFF).

.PARAMETER Exe
    Path to ImageArtBorder.exe (default: same folder as this script).

.PARAMETER WhatIf
    List files that would be processed without changing them.
#>
[CmdletBinding(SupportsShouldProcess = $true)]
param(
    [Parameter(Mandatory = $true)]
    [string] $Folder,

    [double] $Border = 6.0,

    [string] $Color = "#FFFFFF",

    [string] $Exe = "",

    [switch] $WhatIf
)

$ErrorActionPreference = "Stop"

if (-not $Exe) {
    $Exe = Join-Path $PSScriptRoot "ImageArtBorder.exe"
}

if (-not (Test-Path -LiteralPath $Exe)) {
    throw "ImageArtBorder.exe not found: $Exe"
}

$Folder = (Resolve-Path -LiteralPath $Folder).Path
$extensions = @(".jpg", ".jpeg", ".png", ".tif", ".tiff")
$files = Get-ChildItem -LiteralPath $Folder -File -Recurse |
    Where-Object { $extensions -contains $_.Extension.ToLowerInvariant() }

if ($files.Count -eq 0) {
    Write-Warning "No supported images found under $Folder"
    exit 0
}

Write-Host "ImageArtBorder batch - $($files.Count) file(s)"
Write-Host "  Border: $Border% diagonal   Color: $Color"
Write-Host "  Folder: $Folder"
Write-Host ""

$failures = 0
foreach ($file in $files) {
    if ($WhatIf) {
        Write-Host "[WhatIf] $($file.FullName)"
        continue
    }
    Write-Host "Processing: $($file.Name)"
    & $Exe -b $Border -c $Color -f $file.FullName
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Failed: $($file.FullName) (exit $LASTEXITCODE)"
        $failures++
    }
}

if ($WhatIf) {
    Write-Host "WhatIf complete - no files modified."
    exit 0
}

if ($failures -gt 0) {
    Write-Warning "Finished with $failures failure(s)."
    exit 1
}

Write-Host "Done."
exit 0
