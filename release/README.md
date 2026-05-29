# Release packaging

## Build release

From the project root:

```powershell
powershell -ExecutionPolicy Bypass -File .\release\pack-release.ps1
```

Output: `release\ImageArtBorder\` (exe, scripts, docs, `VERSION`).

## Install to `C:\Tools\ImageArtBorder`

```powershell
powershell -ExecutionPolicy Bypass -File .\release\ImageArtBorder\install.ps1
powershell -ExecutionPolicy Bypass -File .\release\ImageArtBorder\install.ps1 -AddToUserPath
```

## Distribute

Zip the folder `release\ImageArtBorder` for end users. They run `install.ps1` after extracting.

**Júlio Papel** — info@juliopapel.pt
