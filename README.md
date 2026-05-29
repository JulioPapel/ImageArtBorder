# ImageArtBorder

**Version 0.2.1** · **Author:** [Júlio Papel](mailto:info@juliopapel.pt) · **License:** [MIT](LICENSE)

Windows command-line tool that adds a solid-color border around photos. Border size scales with each image (**% of diagonal**, default **6%**). Built for **Capture One** export workflows and in-place processing of JPEG, PNG, and TIFF files.

## Install (recommended)

From the project root (developers) or after unpacking a release ZIP:

```powershell
powershell -ExecutionPolicy Bypass -File .\release\pack-release.ps1
powershell -ExecutionPolicy Bypass -File .\release\ImageArtBorder\install.ps1
```

This installs to **`C:\Tools\ImageArtBorder`** (creates the folder, copies exe + scripts + docs, unblocks the exe).

Add to your user **PATH**:

```powershell
powershell -ExecutionPolicy Bypass -File .\release\ImageArtBorder\install.ps1 -AddToUserPath
```

Verify:

```powershell
C:\Tools\ImageArtBorder\ImageArtBorder.exe --version
C:\Tools\ImageArtBorder\ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "D:\path\to\photo.jpg"
```

Uninstall:

```powershell
powershell -ExecutionPolicy Bypass -File C:\Tools\ImageArtBorder\uninstall.ps1
```

## Quick usage

| Task | Command |
|------|---------|
| One file | `ImageArtBorder.exe -b 6 -c "#FFFFFF" -f photo.jpg` |
| Export folder | `Border-ExportedImages.ps1 -Folder "D:\Exports" -Border 6` |
| Help | `ImageArtBorder.exe --help` |

`-b 6` = new diagonal is **6% longer** than the original (pixel border is computed per image).

## Documentation

| Guide | Topic |
|-------|--------|
| [docs/README.md](docs/README.md) | User docs index |
| [docs/installation.md](docs/installation.md) | Install & PATH |
| [docs/capture-one.md](docs/capture-one.md) | Capture One recipe workflow |
| [docs/usage.md](docs/usage.md) | Daily usage |
| [docs/command-reference.md](docs/command-reference.md) | All options |
| [docs/formats-and-preservation.md](docs/formats-and-preservation.md) | JPEG / PNG / TIFF |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Fixes |

[CHANGELOG.md](CHANGELOG.md) · Release folder: [release/](release/)

## Developers

```powershell
.\build.ps1
.\release\pack-release.ps1
cargo test --release --offline
```

| Source | Role |
|--------|------|
| `src/border_calc.rs` | Diagonal % → pixels per side |
| `src/border.rs` | Canvas expansion |
| `src/jpeg_encode.rs` | JPEG + metadata splice |
| `src/formats.rs` | PNG, TIFF, routing |
| `src/metadata/` | Container metadata |

`jpeg-encoder` is vendored under `vendor/` for offline builds.
