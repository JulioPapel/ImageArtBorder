# Command reference

```text
ImageArtBorder.exe [OPTIONS] [IMAGE_PATH]
```

## Options

### `-b`, `--border <PIXELS>`

- Border thickness on **each** of the four sides.
- Default: `40`
- Type: positive integer (pixels).

### `-c`, `--color <HEX>`

- Border fill color.
- Default: `#FFFFFF` (white)
- Forms: `#RRGGBB` or `#AARRGGBB` (alpha is not used for the border fill).

### `-f`, `--file <PATH>`

- Path to the image to process.
- Optional if `IMAGE_PATH` is passed as the last argument.

### `-h`, `--help`

- Show help and exit.

## Positional argument

### `IMAGE_PATH`

- Same as `-f`. If both are given, **`-f` takes precedence**.

## Supported file types

| Extension | Support |
|-----------|---------|
| `.jpg`, `.jpeg`, `.jfif` | Yes |
| `.png` | Yes |
| `.tif`, `.tiff` | Yes |
| `.jxl`, `.jpxl` | Not yet |

Format is detected from the file extension.

## Examples

White border, 40 px (defaults except file):

```powershell
ImageArtBorder.exe -f ".\photo.jpg"
```

Black border, 20 px:

```powershell
ImageArtBorder.exe -b 20 -c "#000000" ".\photo.jpg"
```

Cream border, 60 px:

```powershell
ImageArtBorder.exe -b 60 -c "#F8F4ED" -f "D:\Exports\AP20221112190503.jpg"
```

## Related scripts (release package)

### `Border-ExportedImages.ps1`

| Parameter | Required | Default | Description |
|-----------|----------|---------|-------------|
| `-Folder` | Yes | — | Directory to scan |
| `-Border` | No | `40` | Pixels per side |
| `-Color` | No | `#FFFFFF` | Border color |
| `-Exe` | No | script directory | Path to `ImageArtBorder.exe` |
| `-WhatIf` | No | — | List files only |

### `Add-Border.bat`

- Drag-and-drop or `Add-Border.bat "path\to\file.jpg"`
- Edit `BORDER` and `COLOR` variables inside the file.

## Author and license

**Júlio Papel** — info@juliopapel.pt  
**License:** MIT — see `LICENSE` in the release folder.
