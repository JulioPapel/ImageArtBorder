# ImageArtBorder — User documentation

**ImageArtBorder** adds a uniform colored border around your photographs. It is designed for workflows where **Capture One** (or similar tools) exports the final image and you want a frame without re-editing in Photoshop.

| | |
|---|---|
| **Version** | 0.2.1 |
| **Author** | Júlio Papel |
| **Contact** | [info@juliopapel.pt](mailto:info@juliopapel.pt) |
| **License** | MIT (free) |
| **Install path** | `C:\Tools\ImageArtBorder` |

## Quick start

1. [Install](installation.md) — run `install.ps1` → `C:\Tools\ImageArtBorder`
2. [Usage](usage.md) — command line and batch script
3. [Capture One](capture-one.md) — export recipe workflow

## Guides

- **[Installation](installation.md)** — installer, PATH, uninstall
- **[Usage](usage.md)** — single files and folders
- **[Capture One workflow](capture-one.md)** — export recipes and batch borders
- **[Command reference](command-reference.md)** — `-b`, `-c`, `-f`
- **[Formats and preservation](formats-and-preservation.md)** — JPEG / PNG / TIFF
- **[Troubleshooting](troubleshooting.md)** — common issues

## Typical command

```powershell
C:\Tools\ImageArtBorder\ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\Exports\photo.jpg"
```

Default: **6%** longer diagonal, **white** border, file updated in place.
