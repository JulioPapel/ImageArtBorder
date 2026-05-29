# Installation

ImageArtBorder runs on **Windows 10 or later** (64-bit). No separate runtime (Java, .NET, etc.) is required.

## From the release package

1. Obtain the folder **`ImageArtBorder`** (created by `pack-release.ps1` or distributed as a ZIP). It contains:
   - `ImageArtBorder.exe` — main program
   - `Add-Border.bat` — double-click helper (edit paths inside)
   - `Border-ExportedImages.ps1` — batch border for a whole export folder
   - `LICENSE` — MIT license text
   - `README.txt` — short pointer to this documentation

2. Copy the folder to a permanent location, for example:
   ```
   C:\Program Files\ImageArtBorder\
   ```
   or
   ```
   C:\Tools\ImageArtBorder\
   ```

3. *(Optional)* Add that folder to your **PATH** so you can run `ImageArtBorder` from any terminal:
   - Settings → System → About → Advanced system settings → Environment Variables
   - Under “User variables” or “System variables”, edit **Path**
   - Add: `C:\Tools\ImageArtBorder` (your actual path)

4. Unblock scripts if Windows SmartScreen warns you:
   - Right-click `ImageArtBorder.exe` → Properties → check **Unblock** → OK

## Verify installation

Open **PowerShell** or **Command Prompt**:

```powershell
cd "C:\Tools\ImageArtBorder"
.\ImageArtBorder.exe --help
```

You should see usage for `-b`, `-c`, and `-f`.

## Build from source (developers)

Requires [Rust](https://rustup.rs/). From the project root:

```powershell
.\build.ps1
.\release\pack-release.ps1
```

See the root [README.md](../README.md) for developer details.

## Support

Questions or feedback: **info@juliopapel.pt**
