# Command reference

```text
ImageArtBorder [OPTIONS] [IMAGE_PATH]
```

On Windows the executable is named `ImageArtBorder.exe`; options are identical on macOS and Linux.

## Options

### `-b`, `--border <PERCENT>`

- **Percentage increase of the image diagonal** (not pixels per side).
- Default: `6.0`
- Example: `-b 6` → new diagonal ≈ 106% of the original.
- Must be ≥ 0. Values above 100 are rejected with a warning.

### `-c`, `--color <HEX>`

- Border color: `#RRGGBB` or `#AARRGGBB` (default `#FFFFFF`).

### `-f`, `--file <PATH>`

- Input image (optional if path is the last argument).

### `-h`, `--help` / `-V`, `--version`

- Help and version (semver from the build).

## Examples

**Windows — default (6% diagonal, white)**

```powershell
ImageArtBorder.exe -f ".\photo.jpg"
```

**Windows — 10% diagonal, black**

```powershell
ImageArtBorder.exe -b 10 -c "#000000" ".\photo.jpg"
```

**macOS / Linux**

```bash
ImageArtBorder -f ./photo.jpg
ImageArtBorder -b 10 -c "#000000" ./photo.jpg
./border-exported-images.sh ~/Exports/Finals
```

## Batch scripts

### Windows — `Border-ExportedImages.ps1`

| Parameter | Default | Description |
|-----------|---------|-------------|
| `-Folder` | (required) | Export directory |
| `-Border` | `6` | Diagonal percent (same as `-b`) |
| `-Color` | `#FFFFFF` | Border color |
| `-Exe` | script folder | Path to `ImageArtBorder.exe` |
| `-WhatIf` | — | List files only |

### macOS / Linux — `border-exported-images.sh`

```bash
./border-exported-images.sh <folder> [border_percent] [color_hex]
```

| Argument | Default | Description |
|----------|---------|-------------|
| `folder` | (required) | Directory to scan |
| `border_percent` | `6` | Same as `-b` |
| `color_hex` | `#FFFFFF` | Same as `-c` |

Example:

```bash
./border-exported-images.sh ~/Exports/Finals 8 "#000000"
```

## Author

**Júlio Papel** — info@juliopapel.pt · **License:** MIT
