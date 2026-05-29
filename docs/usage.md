# Usage

## Basic syntax

```text
ImageArtBorder.exe [options] [IMAGE_PATH]
```

The **image path** can be given with **`-f`** or as the **last argument**:

```powershell
ImageArtBorder.exe -b 40 -c "#FFFFFF" -f "D:\Photos\image.jpg"
ImageArtBorder.exe -b 40 -c "#FFFFFF" "D:\Photos\image.jpg"
```

## Options

| Option | Default | Meaning |
|--------|---------|---------|
| `-b`, `--border` | `40` | Border width in **pixels on each side** (top, bottom, left, right) |
| `-c`, `--color` | `#FFFFFF` | Border color: `#RRGGBB` or `#AARRGGBB` |
| `-f`, `--file` | — | Path to the image file |

Final image size: width and height each increase by **`2 × border`**.

Examples:

- Original 3000×2000, `-b 40` → **3080×2080**
- `-b 0` → no change (still rewrites the file through the pipeline)

## In-place update

The program writes to a temporary file (`*.iab.tmp`) and then replaces the original. **Keep backups** of masters until you are satisfied with the result.

## Batch processing (folder)

Use the script included in the release package:

```powershell
cd "C:\Tools\ImageArtBorder"
.\Border-ExportedImages.ps1 -Folder "D:\CaptureOne\Exports\Finals" -Border 40 -Color "#FFFFFF"
```

Supported extensions: `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff`

Add **`-WhatIf`** to list files without changing them.

## Color examples

| Border | Command fragment |
|--------|------------------|
| White | `-c "#FFFFFF"` |
| Black | `-c "#000000"` |
| Warm off-white | `-c "#F5F0E8"` |
| With alpha in hex (alpha ignored for RGB border) | `-c "#FFFFFFFF"` |

Always quote colors that contain `#` in PowerShell.

## Exit codes

- **0** — success  
- **Non-zero** — error message printed (unsupported format, missing file, invalid color, etc.)

See [command-reference.md](command-reference.md) for full detail and [formats-and-preservation.md](formats-and-preservation.md) for JPEG/PNG behavior.
