# Changelog

## [1.0.0] - 2026-05-29

### Added

- **v1.0.0** — first stable public release.
- Automatic **GitHub Releases** on `v*` tags (four platform zip assets).
- Cross-platform documentation and installers (Windows, macOS, Linux).

### Changed

- Version line reaches 1.0.0 after diagonal-percent borders, metadata preservation, and multi-platform packaging (see 0.1.0–0.3.0 below).

## [0.3.0] - 2026-05-29

### Added

- **macOS** (x64 + Apple Silicon) and **Linux x64** native binaries.
- Platform release folders: `release/dist/ImageArtBorder-{version}-{platform}/`
- Unix scripts: `install.sh`, `add-border.sh`, `border-exported-images.sh`
- `scripts/build-linux.sh`, `scripts/build-macos.sh`, `release/build-all.ps1`
- GitHub Actions workflow (`.github/workflows/release.yml`) for all platforms

### Changed

- Windows pack output moved to `release/dist/ImageArtBorder-{version}-windows-x64/` (legacy `release/ImageArtBorder/` still created for install scripts).

## [0.2.1] - 2026-05-29

### Added

- **`install.ps1`** / **`uninstall.ps1`** — install to `C:\Tools\ImageArtBorder` (default) with optional `-AddToUserPath`.
- **`VERSION`** file in the release package.
- Documentation refresh for installer and standard install location.

## [0.2.0] - 2026-05-29

### Changed

- **Border sizing** is now a **percentage of the image diagonal** (default **6.0**), not a fixed pixel width.

### Added

- `src/border_calc.rs` — diagonal-percent border math and unit tests.

## [0.1.0] - 2026-05-29

### Added

- Initial release: JPEG/PNG/TIFF border with metadata preservation.
- Fixed pixel border (default 40 px per side).
- Capture One documentation and `release/` packaging.
