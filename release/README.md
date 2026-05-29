# Release packaging

This folder contains scripts to build the **distributable** `ImageArtBorder` package for end users.

## Create the release

From the **project root**:

```powershell
.\release\pack-release.ps1
```

Output directory:

```text
release\ImageArtBorder\
  ImageArtBorder.exe
  Add-Border.bat
  Border-ExportedImages.ps1
  LICENSE
  README.txt
  docs\          (copied from project docs\)
```

Zip `release\ImageArtBorder` for distribution.

## Author

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)  
**License:** MIT
