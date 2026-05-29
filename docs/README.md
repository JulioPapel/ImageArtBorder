# ImageArtBorder — User documentation

**ImageArtBorder** adds a uniform colored border around your photographs. It is designed for workflows where **Capture One** (or similar tools) exports the final image and you want a frame **without re-editing in Photoshop**—while keeping as much **metadata** and **image quality** as the file format allows.

| | |
|---|---|
| **Version** | 0.3.0 |
| **Platforms** | Windows 10+, macOS 11+, Linux x64 |
| **Author** | Júlio Papel |
| **Contact** | [info@juliopapel.pt](mailto:info@juliopapel.pt) |
| **License** | MIT (free) |
| **Downloads** | [GitHub Releases](https://github.com/JulioPapel/ImageArtBorder/releases) |

## Install locations (after running the installer)

| OS | Default path |
|----|----------------|
| Windows | `C:\Tools\ImageArtBorder\ImageArtBorder.exe` |
| macOS / Linux | `~/.local/bin/ImageArtBorder` |

See [installation.md](installation.md) for full steps.

## Quick start

1. [Installation](installation.md) — download zip, run installer
2. [Usage](usage.md) — one file or whole export folder
3. [Capture One](capture-one.md) — export recipe, then border the folder

### Windows

```powershell
ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\Exports\photo.jpg"
```

### macOS / Linux

```bash
ImageArtBorder -b 6 -c "#FFFFFF" -f ~/Exports/photo.jpg
```

Default: **6%** longer diagonal, **white** border, file updated **in place**.

## Guides

- **[Installation](installation.md)** — GitHub downloads, PATH, uninstall
- **[Usage](usage.md)** — CLI and batch scripts (all platforms)
- **[Capture One workflow](capture-one.md)** — export recipes, then batch border
- **[Command reference](command-reference.md)** — `-b`, `-c`, `-f`, `--version`
- **[Formats and preservation](formats-and-preservation.md)** — JPEG / PNG / TIFF
- **[Troubleshooting](troubleshooting.md)** — Windows, macOS, Linux
- **[Releasing](releasing.md)** — tag push → automatic GitHub Release (maintainers)

## Platform notes

- **Windows** — primary development and testing platform.
- **macOS / Linux** — same features and options; installers use shell scripts. If something fails on your machine, see [troubleshooting.md](troubleshooting.md) and open an issue on GitHub.
