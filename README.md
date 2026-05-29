# ImageArtBorder

**Version 1.0.0** · **Author:** [Júlio Papel](mailto:info@juliopapel.pt) · **License:** [MIT](LICENSE)

CLI that adds a **solid-color border** around photos by **expanding the canvas** (your image stays full size in the center—no downscaling). Border width scales with each file as a **% of the diagonal** (default **6%**). Built for **Capture One** exports and in-place **JPEG / PNG / TIFF** processing, with **best-effort metadata preservation**.

Runs on **Windows**, **macOS**, and **Linux** — single binary, no extra runtime.

## Download

Pre-built zip packages are on [GitHub Releases](https://github.com/JulioPapel/ImageArtBorder/releases). Pick the archive for your system:

| Platform | Release asset |
|----------|----------------|
| Windows x64 | `ImageArtBorder-*-windows-x64.zip` |
| Linux x64 | `ImageArtBorder-*-linux-x64.zip` |
| macOS Apple Silicon | `ImageArtBorder-*-macos-arm64.zip` |
| macOS Intel | `ImageArtBorder-*-macos-x64.zip` |

Unzip, then follow [docs/installation.md](docs/installation.md).

**Testing:** Windows builds are used in production workflows. **macOS and Linux packages are newer and may not yet be verified on every system**—please report issues on GitHub.

## Quick start

**Windows** (installs to `C:\Tools\ImageArtBorder` by default)

```powershell
cd ImageArtBorder-1.0.0-windows-x64
powershell -ExecutionPolicy Bypass -File .\install.ps1
ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "C:\path\to\photo.jpg"
```

**macOS / Linux** (installs to `~/.local/bin`)

```bash
cd ImageArtBorder-1.0.0-macos-arm64   # or linux-x64 / macos-x64
chmod +x ImageArtBorder *.sh
./install.sh
ImageArtBorder -b 6 -c "#FFFFFF" -f ~/Pictures/photo.jpg
```

Batch folder (after export):

```powershell
# Windows
.\Border-ExportedImages.ps1 -Folder "D:\Exports\Finals"
```

```bash
# macOS / Linux
./border-exported-images.sh ~/Exports/Finals
```

## What it does

- **Expands** width and height; original pixels are copied to the center.
- **Preserves metadata** where the format allows (EXIF, XMP, ICC, PNG chunks, JPEG APP segments).
- **Same file format** on output (JPEG in → JPEG out).
- **`-b 6`** means the new diagonal is ~6% longer than the original (pixel border per side varies by image size).

## Documentation

| Guide | Topic |
|-------|--------|
| [docs/README.md](docs/README.md) | User docs index |
| [docs/installation.md](docs/installation.md) | Install per OS, GitHub downloads |
| [docs/usage.md](docs/usage.md) | Commands and batch scripts |
| [docs/capture-one.md](docs/capture-one.md) | Capture One export workflow |
| [docs/command-reference.md](docs/command-reference.md) | All options |
| [docs/formats-and-preservation.md](docs/formats-and-preservation.md) | JPEG / PNG / TIFF behavior |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Common fixes |

[CHANGELOG.md](CHANGELOG.md)

## Build from source

Requires [Rust](https://rustup.rs/).

| Host | Command | Output |
|------|---------|--------|
| Windows | `.\release\pack-release.bat` | `release/dist/...-windows-x64/` |
| Windows (all via Docker) | `.\release\build-all.bat` | Windows + Linux + macOS |
| Linux | `./scripts/build-linux.sh` | `...-linux-x64/` |
| macOS | `./scripts/build-macos.sh` | `...-macos-arm64/` and/or `...-macos-x64/` |

Zip for GitHub: `.\release\pack-zips.bat` → `release/zips/*.zip`

If PowerShell blocks `.ps1` scripts, use the `.bat` launchers above or run:  
`powershell -ExecutionPolicy Bypass -File .\release\build-all.ps1`

### Automatic GitHub Releases

Push a version tag to build all platforms and publish release zips:

```bash
git tag v1.0.0
git push origin v1.0.0
```

See [docs/releasing.md](docs/releasing.md) and [.github/workflows/release.yml](.github/workflows/release.yml).

## Developers

```powershell
cargo test --release --offline
```

| Module | Role |
|--------|------|
| `src/border_calc.rs` | Diagonal % → pixel border |
| `src/jpeg_encode.rs` | JPEG encode + metadata splice |
| `scripts/` | Unix / Docker macOS / Linux pack |
| `release/` | Windows pack, installers, zip helper |
