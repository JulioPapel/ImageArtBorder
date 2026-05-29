# Formats and preservation

ImageArtBorder **expands** the image canvas and fills the new edge area with your border color. The original picture sits in the center; it is not scaled down to “fit inside a mat.”

## What “preserved” means

| Aspect | Goal |
|--------|------|
| **Metadata** | Copy unknown and known sidecar data (EXIF, XMP, ICC, Photoshop markers, PNG text chunks, etc.) without rewriting through a high-level editor API where possible |
| **Interior pixels** | Copy source samples exactly into the larger buffer before saving |
| **File format** | Same extension and general format (JPEG stays JPEG, etc.) |

## JPEG

- **APP0–APP15** and **COM** segments: copied **byte-for-byte** from the source.
- **DQT** (quantization tables): copied from the source.
- **SOF / DHT / scan data**: regenerated for the new dimensions (required for a valid larger JPEG).
- **Pixels**: Interior is copied in memory before encode, using the source quantization tables and chroma subsampling. Lossy JPEG may show tiny value changes (typical of any re-save) even in the center; this is a limitation of the JPEG standard when size changes.

For archival masters, keep an **untouched backup** or export **TIFF/PNG** from Capture One if you need zero generation loss.

## PNG

- All chunks except **IHDR**, **IDAT**, and **IEND** are copied from the source (including custom/unknown chunks).
- New IHDR reflects larger dimensions; new IDAT holds the expanded image.
- Lossless formats can preserve interior pixels exactly.

## TIFF

- Image data is expanded and rewritten; TIFF tag round-trip is limited compared to JPEG/PNG metadata handling.

## Not supported

- **JPEG XL** (`.jxl`, `.jpxl`) — planned for a future release.

## In-place write

The tool replaces the original file using a temporary `*.iab.tmp` file, then renames. If the process is interrupted, check for leftover `.iab.tmp` files next to your images.
