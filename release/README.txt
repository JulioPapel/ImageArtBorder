ImageArtBorder v0.3.0
====================
Author: Júlio Papel
Email:  info@juliopapel.pt
License: MIT (free) — see LICENSE

INSTALL (recommended)
-------------------
  powershell -ExecutionPolicy Bypass -File .\install.ps1

  Installs to: C:\Tools\ImageArtBorder
  Optional PATH:  .\install.ps1 -AddToUserPath

QUICK START
-----------
  C:\Tools\ImageArtBorder\ImageArtBorder.exe --version
  C:\Tools\ImageArtBorder\ImageArtBorder.exe -b 6 -c "#FFFFFF" -f "C:\path\to\photo.jpg"

  Capture One export folder:
  C:\Tools\ImageArtBorder\Border-ExportedImages.ps1 -Folder "D:\Exports\Finals" -Border 6

  (-b 6 = image diagonal 6% longer; pixel border varies by image size)

UNINSTALL
---------
  powershell -ExecutionPolicy Bypass -File C:\Tools\ImageArtBorder\uninstall.ps1

DOWNLOAD / OTHER PLATFORMS
--------------------------
  https://github.com/JulioPapel/ImageArtBorder/releases
  (Linux, macOS arm64, macOS x64)

DOCUMENTATION
-------------
  docs\README.md
  docs\installation.md
  docs\capture-one.md
  docs\usage.md
  docs\troubleshooting.md

SUPPORT: info@juliopapel.pt
