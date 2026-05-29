# ImageArtBorder

**Author:** Júlio Papel — [info@juliopapel.pt](mailto:info@juliopapel.pt)  
**License:** [MIT](LICENSE) (free to use, modify, and distribute)

Windows command-line tool that adds a solid-color border around exported photos while preserving metadata and the original image area as faithfully as the file format allows.

## End users

See the **[docs/](docs/)** folder:

| Document | Contents |
|----------|----------|
| [docs/README.md](docs/README.md) | Documentation index |
| [docs/installation.md](docs/installation.md) | Install from the release package |
| [docs/usage.md](docs/usage.md) | Command-line usage |
| [docs/capture-one.md](docs/capture-one.md) | Capture One export recipe workflow |
| [docs/command-reference.md](docs/command-reference.md) | All options |
| [docs/formats-and-preservation.md](docs/formats-and-preservation.md) | JPEG / PNG / TIFF behavior |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Common issues |

Pre-built files (after packaging): **[release/](release/)** — run `.\release\pack-release.ps1` from the project root.

See **[CHANGELOG.md](CHANGELOG.md)** for version history.

## Developers

### Build

```powershell
.\build.ps1              # compile release binary
.\release\pack-release.ps1   # build + copy into release\ImageArtBorder\
```

Output executable: `target\release\ImageArtBorder.exe`

### Tests

```powershell
$env:CARGO_HTTP_CHECK_REVOCATION='false'
cargo test --release --offline
```

### Source layout

| Path | Role |
|------|------|
| `src/main.rs` | Entry point |
| `src/cli.rs` | Command-line arguments |
| `src/border.rs` | Canvas expansion and pixel copy |
| `src/jpeg_encode.rs` | JPEG decode/encode and metadata splice |
| `src/formats.rs` | PNG, TIFF, routing |
| `src/metadata/` | JPEG/PNG container metadata preservation |

`jpeg-encoder` is vendored under `vendor/` for offline builds.
