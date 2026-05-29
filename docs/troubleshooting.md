# Troubleshooting

## “no image path” or missing file

- Provide **`-f`** with the full path, or put the path **last** on the command line.
- Use quotes if the path contains spaces or special characters.

## “unsupported format”

- Extensions: `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff` only.
- JPEG XL (`.jxl`) is not supported yet.

## “invalid color hex”

- Use `#RRGGBB`, e.g. `#FFFFFF`.
- In PowerShell, quote the color: `-c "#FFFFFF"`.

---

## Windows

### SmartScreen or “Windows protected your PC”

1. Right-click `ImageArtBorder.exe` → **Properties** → check **Unblock** if shown.
2. Or allow the app once in **Windows Security**.

### PowerShell script will not run (“running scripts is disabled”)

**Quick fix (one command):**

```powershell
powershell -ExecutionPolicy Bypass -File .\release\build-all.ps1
```

**Or use the `.bat` wrappers** (no policy change needed):

```cmd
.\release\build-all.bat
.\release\pack-release.bat
.\release\pack-zips.bat
```

**Optional — allow scripts for your user account:**

```powershell
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
```

Then `install.ps1`, `build-all.ps1`, and `Border-ExportedImages.ps1` run normally.

### `Border-ExportedImages.ps1` finds no files

- Extensions must be `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff`.
- Use `-WhatIf` to list matches without processing.

---

## macOS

### `command not found: ImageArtBorder`

- Run `./install.sh` from the release folder.
- Ensure `~/.local/bin` is on PATH (see [installation.md](installation.md)).

### “cannot be opened” / Gatekeeper

```bash
xattr -dr com.apple.quarantine /path/to/ImageArtBorder
```

Or allow in **System Settings → Privacy & Security**.

### `Permission denied` on scripts

```bash
chmod +x ImageArtBorder install.sh uninstall.sh border-exported-images.sh add-border.sh
```

### Binary runs but crashes or wrong arch

- Apple Silicon → use **macos-arm64** package.
- Intel Mac → use **macos-x64** package.
- Report OS version and chip on GitHub if problems persist.

---

## Linux

### `command not found: ImageArtBorder`

- Run `./install.sh`; confirm `~/.local/bin` is in PATH.
- Log out and back in, or `source ~/.profile` / `~/.bashrc`.

### `Permission denied`

```bash
chmod +x ImageArtBorder *.sh
```

### glibc / distro issues

- Package is built for **x86_64** with typical glibc (Ubuntu/Debian-style CI/Docker).
- On Alpine or musl-only systems, build from source: `./scripts/build-linux.sh`.

---

## All platforms

### Border looks wrong after Capture One export

- Run ImageArtBorder **after** export ([capture-one.md](capture-one.md)).
- Remember **`-b`** is **percent of diagonal** (default `6`), not pixels.
- Test one file, read the stderr border line, then batch the folder.

### Image corrupted or solid color

- Often an interrupted write. Restore from backup.
- Delete leftover `*.iab.tmp` next to the file; retry on a **copy**.

### JPEG center color slightly different

- Normal for **lossy JPEG** when dimensions change; interior is copied before encode.
- For zero loss, export **PNG/TIFF** from Capture One or keep a non-bordered master.

### Batch script processes nothing

- Check folder path and file extensions.
- Windows: `-WhatIf` on `Border-ExportedImages.ps1`.

---

## Contact

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)
