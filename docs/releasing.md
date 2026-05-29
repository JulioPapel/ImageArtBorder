# Releasing a new version

GitHub Actions creates a **Release** automatically when you push a **version tag**.

## Steps

1. **Bump version** in `Cargo.toml` and add a section to `CHANGELOG.md`.
2. **Commit and push** to `main`:
   ```bash
   git add Cargo.toml Cargo.lock CHANGELOG.md
   git commit -m "v0.3.1: short description"
   git push origin main
   ```
3. **Tag and push** the tag (this triggers the release workflow):
   ```bash
   git tag v0.3.1
   git push origin v0.3.1
   ```

4. Open **Actions** on GitHub → **Release** workflow. When it finishes, the new release appears under **Releases** with four zip files:
   - `ImageArtBorder-{version}-windows-x64.zip`
   - `ImageArtBorder-{version}-linux-x64.zip`
   - `ImageArtBorder-{version}-macos-arm64.zip`
   - `ImageArtBorder-{version}-macos-x64.zip`

## Tag rules

- Tag must match `v*` (e.g. `v0.3.0`, `v1.0.0`).
- Tag should match `version` in `Cargo.toml` (without the `v` prefix).

## Manual workflow run

**Actions → Release → Run workflow** builds all four platforms and uploads artifacts, but **does not** create a GitHub Release. Use this to test CI; publish with a tag push.

## Local builds (optional)

Same zips as CI:

```cmd
.\release\build-all.bat
.\release\pack-zips.bat
```

Output: `release\zips\`

## First-time setup

- Repository **Settings → Actions → General** → allow workflows to run.
- Default `GITHUB_TOKEN` is enough; the workflow sets `contents: write` to publish releases.
