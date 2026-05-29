# Installation

ImageArtBorder runs on **Windows 10+**, **macOS 11+**, and **Linux x64** (glibc). No Java, .NET, or other runtime is required—only the single executable in the release package.

## Get the software

### GitHub Releases (recommended)

1. Open [Releases](https://github.com/JulioPapel/ImageArtBorder/releases).
2. Download the zip for your platform:

| Your system | Asset name |
|-------------|------------|
| Windows 64-bit | `ImageArtBorder-{version}-windows-x64.zip` |
| Linux 64-bit | `ImageArtBorder-{version}-linux-x64.zip` |
| Mac (M1/M2/M3…) | `ImageArtBorder-{version}-macos-arm64.zip` |
| Mac (Intel) | `ImageArtBorder-{version}-macos-x64.zip` |

3. Unzip to a folder you control (e.g. `Downloads\ImageArtBorder-1.0.0-windows-x64`).
4. Follow the OS section below.

Each package includes `docs/`, `LICENSE`, `VERSION`, the binary, install scripts, and batch helpers.

### Build from source

See [Build from source](#build-from-source-developers) at the bottom of this page, or the root [README.md](../README.md).

---

## Windows

**Default install path:** `C:\Tools\ImageArtBorder`

```powershell
cd path\to\ImageArtBorder-{version}-windows-x64
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

Add to your user **PATH** (optional):

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -AddToUserPath
```

Verify:

```powershell
C:\Tools\ImageArtBorder\ImageArtBorder.exe --version
C:\Tools\ImageArtBorder\ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "C:\path\to\photo.jpg"
```

**Uninstall:** run `uninstall.ps1` from `C:\Tools\ImageArtBorder`, or from the unzipped package folder.

**First run:** if Windows SmartScreen blocks the exe, see [troubleshooting.md](troubleshooting.md).

---

## macOS

Use **arm64** on Apple Silicon, **x64** on Intel Macs.

```bash
cd ~/Downloads/ImageArtBorder-1.0.0-macos-arm64   # adjust path and arch
chmod +x ImageArtBorder *.sh
./install.sh
```

This copies the binary to **`~/.local/bin/ImageArtBorder`**.

Ensure `~/.local/bin` is on your PATH (common on modern macOS with a login shell):

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zprofile
source ~/.zprofile
```

Verify:

```bash
ImageArtBorder --version
ImageArtBorder -b 6 -c "#FFFFFF" -f ~/Pictures/photo.jpg
```

**Batch folder after Capture One export:**

```bash
./border-exported-images.sh ~/Exports/Finals
```

**Uninstall:** `./uninstall.sh` from the package folder (or the copy next to the binary).

**Gatekeeper:** if macOS says the app cannot be opened, allow it in **System Settings → Privacy & Security**, or remove quarantine once:

```bash
xattr -dr com.apple.quarantine /path/to/ImageArtBorder
```

**Note:** macOS builds may be cross-compiled; if the binary does not run on your Mac, please report your OS version and chip (Intel / Apple Silicon) on GitHub.

---

## Linux

```bash
cd ~/Downloads/ImageArtBorder-1.0.0-linux-x64
chmod +x ImageArtBorder *.sh
./install.sh
```

Installs to **`~/.local/bin/ImageArtBorder`**. Add `~/.local/bin` to PATH if needed (same as macOS above).

Verify:

```bash
ImageArtBorder --version
ImageArtBorder -b 6 -c "#FFFFFF" -f /path/to/photo.jpg
./border-exported-images.sh /path/to/export/folder
```

**Uninstall:** `./uninstall.sh`

**Note:** Linux packages are tested via build pipelines; report distro-specific issues (Ubuntu, Fedora, etc.) on GitHub.

---

## After installation

| Task | Windows | macOS / Linux |
|------|---------|----------------|
| One image | `ImageArtBorder.exe -f path` | `ImageArtBorder -f path` |
| Export folder | `Border-ExportedImages.ps1 -Folder path` | `./border-exported-images.sh path` |
| Capture One flow | [capture-one.md](capture-one.md) | same guide (use shell batch script) |

---

## Build from source (developers)

Requires [Rust](https://rustup.rs/).

| Host | Command | Result |
|------|---------|--------|
| Windows | `.\release\pack-release.bat` | `release/dist/ImageArtBorder-{ver}-windows-x64/` |
| Windows (Linux + macOS via Docker) | `.\release\build-all.bat` | all `release/dist/...` folders |
| Linux only (Docker on Windows) | `.\scripts\build-linux-docker.ps1` | `...-linux-x64/` |
| macOS only (Docker on Windows) | `.\scripts\build-macos-docker.ps1` | `...-macos-arm64/` + `...-macos-x64/` |
| Linux native | `./scripts/build-linux.sh` | `...-linux-x64/` |
| macOS native | `./scripts/build-macos.sh` | native arch + optional other arch |

Create GitHub zip attachments:

```cmd
.\release\pack-zips.bat
```

If `.ps1` files are blocked, use the `.bat` files or:  
`powershell -ExecutionPolicy Bypass -File .\release\build-all.ps1`

Output: `release/zips/ImageArtBorder-{version}-{platform}.zip`

Pushing a `v*` git tag triggers GitHub Actions to build all four platforms (see `.github/workflows/release.yml`).

---

## Support

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)
