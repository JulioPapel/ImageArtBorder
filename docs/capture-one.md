# Capture One — export with border

Capture One does not run arbitrary command-line tools **inside** an export recipe. The reliable approach is:

1. **Export** with a dedicated recipe (format, size, color space, output folder).
2. **Run ImageArtBorder** on that folder (script or watch), so each exported file gets the border **after** processing.

This matches a typical studio flow: Capture One produces the final RGB file; ImageArtBorder only expands the canvas and fills the border.

---

## Step 1 — Install ImageArtBorder

See [installation.md](installation.md). Note the full path to `ImageArtBorder.exe`, for example:

```text
C:\Tools\ImageArtBorder\ImageArtBorder.exe
```

(Install with `install.ps1` from the release package — see [installation.md](installation.md).)

---

## Step 2 — Create an export recipe in Capture One

1. Select the variants you will export.
2. Open the **Export** dialog:
   - Toolbar **Export**, or  
   - **File → Export → Export…**, or  
   - **Shift + Ctrl + D** (Windows).

3. In **Export Recipes**, click **+** (New recipe) and name it, for example:
   ```text
   Finals — JPEG sRGB (border pending)
   ```

4. On the **Basic** (or equivalent) tab, set for example:

   | Setting | Suggested value |
   |-----------|-----------------|
   | Format | JPEG (or TIFF for maximum quality) |
   | Quality | 90–100 |
   | Color profile | sRGB (or your delivery profile) |
   | Scale | Fixed 100% or your delivery long edge |
   | Output location | A **dedicated folder**, e.g. `D:\CaptureOne\Exports\Finals` |

5. **Open With** — leave **empty** for batch exports.  
   (Opening each file in another app after export slows large jobs; use the batch script instead.)

6. Save the recipe. Enable only this recipe when you want bordered finals.

7. Export (**Export** button or **Ctrl + D** / process shortcut depending on your Capture One version).

All files land in your output folder **without** a border yet.

---

## Step 3 — Add borders after export (recommended)

### Option A — PowerShell batch script (best for many files)

Edit `Border-ExportedImages.ps1` in the release folder if needed, then run:

```powershell
& "C:\Tools\ImageArtBorder\Border-ExportedImages.ps1" `
  -Folder "D:\CaptureOne\Exports\Finals" `
  -Border 6 `
  -Color "#FFFFFF"
```

Preview first:

```powershell
& "C:\Tools\ImageArtBorder\Border-ExportedImages.ps1" -Folder "D:\CaptureOne\Exports\Finals" -WhatIf
```

### Option B — One file with the launcher batch file

After a single export, drag the file onto **`Add-Border.bat`** (configure default `-b` and `-c` inside the batch file), or run:

```cmd
"C:\Tools\ImageArtBorder\Add-Border.bat" "D:\CaptureOne\Exports\Finals\image.jpg"
```

### Option C — “Open With” for single images only

Only practical for **one image at a time**:

1. Create a small launcher, e.g. `OpenWith-Border.bat`:
   ```bat
   @"C:\Tools\ImageArtBorder\ImageArtBorder.exe" -b 6 -c "#FFFFFF" -f "%~1"
   ```
2. In the export recipe, set **Open With** to that batch file *(if Capture One lists it)*.

Capture One may not pass file paths to all “Open With” targets on Windows; if the border does not run, use Option A instead.

---

## Step 4 — Optional automation (watch folder)

For hands-free runs after each export session:

1. Keep exporting to the same folder, e.g. `D:\CaptureOne\Exports\Finals`.
2. Use **Task Scheduler** or run this in PowerShell after you finish exporting:

   ```powershell
   while ($true) {
     & "C:\Tools\ImageArtBorder\Border-ExportedImages.ps1" -Folder "D:\CaptureOne\Exports\Finals" -Border 6 -Color "#FFFFFF"
     Start-Sleep -Seconds 60
   }
   ```

   Stop with **Ctrl+C** when done. Adjust sleep interval to avoid overlapping runs on huge batches.

---

## Recipe checklist (summary)

| Step | Action |
|------|--------|
| 1 | Recipe: JPEG/TIFF, correct profile and size |
| 2 | Output folder: fixed path you control |
| 3 | Open With: **off** for batches |
| 4 | Export variants |
| 5 | Run `Border-ExportedImages.ps1` on that folder |
| 6 | Deliver bordered files from the same folder |

---

## Border size and print

`-b 6` means the **diagonal grows by 6%** (not 6 pixels). Border width in pixels depends on image size:

- A 3393×2500 export gets a wider pixel border than an 800×600 proof.
- Run one test file and read the stderr line (`border Npx/side`) before batching hundreds of images.
- For a different look, try `-b 4` or `-b 8`.

---

## Metadata

ImageArtBorder copies EXIF/XMP and other marker data from JPEG/PNG where possible. See [formats-and-preservation.md](formats-and-preservation.md).

---

## Contact

**Júlio Papel** — [info@juliopapel.pt](mailto:info@juliopapel.pt)
