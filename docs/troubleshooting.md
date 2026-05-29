# Troubleshooting

## “no image path” or missing file

- Provide **`-f "full\path\to\file.jpg"`** or put the path **last** on the command line.
- Use quotes if the path contains spaces or special characters (e.g. `© Júlio Papel`).

## “unsupported format”

- Check the extension: `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff` only.
- JPEG XL (`.jxl`) is not supported yet.

## “invalid color hex”

- Use `#RRGGBB`, e.g. `#FFFFFF`.
- In PowerShell, quote the color: `-c "#FFFFFF"`.

## Windows blocks the .exe

1. Right-click `ImageArtBorder.exe` → **Properties** → **Unblock**.
2. Or allow the app in Windows Security / SmartScreen once.

## PowerShell script will not run

```powershell
Set-ExecutionPolicy -Scope CurrentUser -ExecutionPolicy RemoteSigned
```

Then run the script again. Only change policy if you trust this package.

## Border looks wrong in Capture One export

- Confirm you ran ImageArtBorder **after** export (see [capture-one.md](capture-one.md)).
- Check **`-b`** is a **percent of diagonal** (default `6`), not pixels. Test one file, then adjust.
- Ensure **`-c`** is quoted in PowerShell.

## Image looks corrupted or solid color

- Usually caused by an interrupted write. Restore from backup.
- Delete any `*.iab.tmp` next to the file and re-run on a copy.

## JPEG color slightly different in center

- Expected for **lossy JPEG** when dimensions change; interior is still copied before encode with source tables.
- For zero loss, use **PNG/TIFF** export from Capture One or keep a non-bordered master.

## Batch script finds no files

- Extensions must be `.jpg`, `.jpeg`, `.png`, `.tif`, `.tiff` (case-insensitive).
- Use `-WhatIf` to see what would be processed.

## Contact

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)
