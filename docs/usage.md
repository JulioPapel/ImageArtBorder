# Usage

## Command name

| Platform | Executable |
|----------|------------|
| Windows | `ImageArtBorder.exe` |
| macOS / Linux | `ImageArtBorder` |

Syntax is the same on all systems.

```text
ImageArtBorder [OPTIONS] [IMAGE_PATH]
```

## Single image

**Windows**

```powershell
ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\Photos\image.jpg"
ImageArtBorder.exe -b 6 -c "#FFFFFF" "D:\Photos\image.jpg"
```

**macOS / Linux**

```bash
ImageArtBorder -b 6 -c "#FFFFFF" -f ~/Photos/image.jpg
ImageArtBorder -b 6 -c "#FFFFFF" ~/Photos/image.jpg
```

You can omit `-f` if the file path is the **last** argument.

## Border size (`-b`)

`-b` is a **percentage of the image diagonal**, not pixels per side.

| Value | Meaning |
|-------|---------|
| `6` (default) | New diagonal is **6% longer** than the original |
| `0` | No border |
| `10` | New diagonal is **10% longer** |

The program computes **pixels per side** for each image. A 3393×2500 export and an 800×600 proof both use `-b 6`, but the pixel border width differs.

Example log line (stderr):

```text
Image 3393x2500: border 126px/side (6.0% diagonal: 4221 -> 4474 px)
```

## Color (`-c`)

Default: `#FFFFFF` (white). Use `#RRGGBB` or `#AARRGGBB`.

- **PowerShell:** quote the color: `-c "#000000"`
- **bash/zsh:** quotes are optional for simple hex values

## In-place update

The original file is replaced via a temporary `*.iab.tmp` file, then renamed. Keep backups of masters when trying new settings.

## Batch processing (export folder)

After Capture One (or any tool) has written files to a folder, run the batch helper on that directory.

**Windows**

```powershell
cd C:\Tools\ImageArtBorder
.\Border-ExportedImages.ps1 -Folder "D:\CaptureOne\Exports\Finals" -Border 6 -Color "#FFFFFF"
```

Preview only:

```powershell
.\Border-ExportedImages.ps1 -Folder "D:\CaptureOne\Exports\Finals" -WhatIf
```

**macOS / Linux** (from the release folder or install directory)

```bash
./border-exported-images.sh ~/Exports/Finals
./border-exported-images.sh ~/Exports/Finals 8 "#000000"   # 8% diagonal, black
```

Supported extensions: `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff` (case-insensitive).

## Windows extras

- **`Add-Border.bat`** — drag one file onto the batch file (edit default `-b` / `-c` inside the file).
- Installed copy lives in `C:\Tools\ImageArtBorder\` after `install.ps1`.

## See also

- [command-reference.md](command-reference.md) — all flags and script parameters
- [capture-one.md](capture-one.md) — recipe + export + batch border
- [formats-and-preservation.md](formats-and-preservation.md) — what is preserved per format
