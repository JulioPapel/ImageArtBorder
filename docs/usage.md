# Usage

## Basic syntax

```text
ImageArtBorder.exe [options] [IMAGE_PATH]
```

```powershell
ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\Photos\image.jpg"
ImageArtBorder.exe -b 6 -c "#FFFFFF" "D:\Photos\image.jpg"
```

## Border size (`-b`)

`-b` is a **percentage of the image diagonal**, not pixels.

| Value | Meaning |
|-------|---------|
| `6` (default) | New diagonal is **6% longer** than the original |
| `0` | No border |
| `10` | New diagonal is **10% longer** |

The program computes how many **pixels per side** are needed for your image size. A 3000×2000 export and a 800×600 export both use `-b 6`, but the pixel border width differs.

Example output:

```text
Image 3393x2500: border 126px/side (6.0% diagonal: 4221 -> 4474 px)
```

## Color (`-c`)

Default: `#FFFFFF` (white). Use quotes in PowerShell: `-c "#000000"`.

## In-place update

The file is replaced via a temporary `*.iab.tmp` file. Keep backups of masters when trying new settings.

## Batch processing

```powershell
.\Border-ExportedImages.ps1 -Folder "D:\CaptureOne\Exports\Finals" -Border 6 -Color "#FFFFFF"
```

## See also

- [command-reference.md](command-reference.md)
- [capture-one.md](capture-one.md)
- [formats-and-preservation.md](formats-and-preservation.md)
