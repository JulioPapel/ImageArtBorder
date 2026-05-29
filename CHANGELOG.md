# Changelog

## [0.2.0] - 2026-05-29

### Changed

- **Border sizing** is now a **percentage of the image diagonal** (default **6.0**), not a fixed pixel width.
  - `-b 6` means the new diagonal is **6% longer** than the original (uniform border on all sides).
  - The tool prints the computed pixel border and actual diagonal change per file.

### Added

- `src/border_calc.rs` — diagonal-percent border math and unit tests.

## [0.1.0] - 2026-05-29

### Added

- Initial release: JPEG/PNG/TIFF border with metadata preservation.
- Fixed pixel border (default 40 px per side).
- Capture One documentation and `release/` packaging.
