# Installation

ImageArtBorder runs on **Windows 10 or later** (64-bit). No Java, .NET, or other runtime is required.

## Standard location: `C:\Tools\ImageArtBorder`

The recommended install path is:

```text
C:\Tools\ImageArtBorder\
```

## Automatic install (recommended)

### From a release package

After unpacking the `ImageArtBorder` folder (or building with `pack-release.ps1`):

```powershell
cd path\to\ImageArtBorder
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

### With PATH (optional)

```powershell
powershell -ExecutionPolicy Bypass -File .\install.ps1 -AddToUserPath
```

Close and reopen the terminal, then run:

```powershell
ImageArtBorder.exe --version
```

### Custom install folder

```powershell
.\install.ps1 -InstallDir "D:\Apps\ImageArtBorder"
```

## What gets installed

| Item | Purpose |
|------|---------|
| `ImageArtBorder.exe` | Main program |
| `Add-Border.bat` | Drag-and-drop single file |
| `Border-ExportedImages.ps1` | Batch folder (Capture One exports) |
| `install.ps1` / `uninstall.ps1` | Install or remove |
| `VERSION` | Installed version string |
| `docs\` | Full user documentation |
| `LICENSE` | MIT license |

## Verify

```powershell
C:\Tools\ImageArtBorder\ImageArtBorder.exe --help
C:\Tools\ImageArtBorder\ImageArtBorder.exe --version
```

You should see **0.2.1** (or your build version) and options `-b`, `-c`, `-f`.

## Uninstall

```powershell
powershell -ExecutionPolicy Bypass -File C:\Tools\ImageArtBorder\uninstall.ps1
```

Or delete `C:\Tools\ImageArtBorder` manually and remove it from **PATH** in Environment Variables.

## Manual install

Copy the entire `ImageArtBorder` release folder to `C:\Tools\ImageArtBorder`. Right-click `ImageArtBorder.exe` → **Properties** → **Unblock** if Windows SmartScreen blocked it.

## Build from source

Requires [Rust](https://rustup.rs/):

```powershell
git clone <your-repo-url>
cd ImageArtBorder
.\release\pack-release.ps1
.\release\ImageArtBorder\install.ps1
```

## Support

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)
