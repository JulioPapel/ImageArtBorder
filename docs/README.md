# ImageArtBorder — User documentation

**ImageArtBorder** adds a uniform colored border around your photographs. It is designed for workflows where another program (for example **Capture One**) exports the final image and you want a white or custom-color frame without re-editing in Photoshop.

| | |
|---|---|
| **Author** | Júlio Papel |
| **Contact** | [info@juliopapel.pt](mailto:info@juliopapel.pt) |
| **License** | MIT (free) |

## Quick start

1. [Install](installation.md) — copy the `release\ImageArtBorder` folder to your PC.
2. [Usage](usage.md) — run from Command Prompt or PowerShell.
3. [Capture One](capture-one.md) — tie borders into your export recipe.

## Guides

- **[Installation](installation.md)** — system requirements and setup
- **[Usage](usage.md)** — single files and folders
- **[Capture One workflow](capture-one.md)** — export recipes, batch scripts, recommended settings
- **[Command reference](command-reference.md)** — `-b`, `-c`, `-f`, examples
- **[Formats and preservation](formats-and-preservation.md)** — what is kept for JPEG, PNG, TIFF
- **[Troubleshooting](troubleshooting.md)** — errors and fixes

## Typical command

```powershell
ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\Exports\photo.jpg"
```

Default border is **6%** of the image diagonal (new diagonal is 6% longer), color **white** (`#FFFFFF`). The file is updated in place (a temporary file is used, then renamed).
