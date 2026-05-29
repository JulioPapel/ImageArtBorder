# Command reference

```text
ImageArtBorder.exe [OPTIONS] [IMAGE_PATH]
```

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

- Help and version (includes semver from the build).

## Examples

Default (6% diagonal, white):

```powershell
ImageArtBorder.exe -f ".\photo.jpg"
```

10% diagonal, black border:

```powershell
ImageArtBorder.exe -b 10 -c "#000000" ".\photo.jpg"
```

## `Border-ExportedImages.ps1`

| Parameter | Default | Description |
|-----------|---------|-------------|
| `-Folder` | (required) | Export directory |
| `-Border` | `6` | Diagonal percent (same as `-b`) |
| `-Color` | `#FFFFFF` | Border color |
| `-Exe` | script folder | Path to `ImageArtBorder.exe` |
| `-WhatIf` | — | List files only |

## Author

**Júlio Papel** — info@juliopapel.pt · **License:** MIT
