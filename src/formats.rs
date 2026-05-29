//! Format detection, per-format encode pipelines, and atomic in-place write.

use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::tiff::TiffEncoder;
use image::{ExtendedColorType, GenericImageView, ImageEncoder};
use jpeg_decoder::Decoder;

use crate::border;
use crate::border_calc::{diagonals, pixels_from_diagonal_percent};
use crate::cli::{parse_color, Args};
use crate::jpeg_encode;
use crate::metadata::finalize_png;

/// Supported container formats (from file extension).
#[derive(Clone, Copy, PartialEq, Eq)]
enum ImageFormat {
    Jpeg,
    Png,
    Tiff,
    Jxl,
}

/// Read the input file, add a border, and replace the original atomically.
pub fn process(args: &Args) -> Result<()> {
    args.validate_border_percent()?;
    let path = args.image_path()?.to_path_buf();
    let color = parse_color(&args.color)?;

    let format = detect_format(&path)?;
    let source_bytes = fs::read(&path).with_context(|| format!("read {}", path.display()))?;

    let (width, height) = image_dimensions(format, &source_bytes)?;
    let border_px = pixels_from_diagonal_percent(width, height, args.border_percent);

    if border_px > 0 {
        let (d_old, d_new) = diagonals(width, height, border_px);
        let pct_actual = (d_new / d_old - 1.0) * 100.0;
        eprintln!(
            "Image {}x{}: border {border_px}px/side ({:.1}% diagonal: {d_old:.0} -> {d_new:.0} px)",
            width, height, pct_actual
        );
    }

    let output_bytes = match format {
        ImageFormat::Jpeg => jpeg_encode::process_jpeg(&source_bytes, border_px, color)?,
        ImageFormat::Png => process_png(&source_bytes, border_px, color)?,
        ImageFormat::Tiff => process_tiff(&source_bytes, border_px, color)?,
        ImageFormat::Jxl => {
            bail!(
                "JPEG XL ({}) is not supported yet",
                path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("jxl")
            )
        }
    };

    write_atomic(&path, &output_bytes)?;
    Ok(())
}

/// Image width and height before border is applied.
fn image_dimensions(format: ImageFormat, source: &[u8]) -> Result<(u32, u32)> {
    match format {
        ImageFormat::Jpeg => {
            let mut dec = Decoder::new(Cursor::new(source));
            dec.read_info().context("read JPEG dimensions")?;
            let info = dec.info().context("JPEG missing image info")?;
            Ok((info.width as u32, info.height as u32))
        }
        _ => {
            let img = image::load_from_memory(source).context("decode image for dimensions")?;
            Ok(img.dimensions())
        }
    }
}

/// Write via a sibling `.iab.tmp` file, then rename over the original.
fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    let tmp = path.with_extension("iab.tmp");
    fs::write(&tmp, data)?;
    fs::rename(&tmp, path).with_context(|| format!("replace {}", path.display()))?;
    Ok(())
}

/// PNG: expand pixels, encode, then copy ancillary chunks from the source.
fn process_png(source: &[u8], border: u32, color: rgb::RGB8) -> Result<Vec<u8>> {
    let img = image::load_from_memory(source).context("decode PNG")?;
    let expanded = border::expand(img, border, color)?;
    let encoded = encode_png(&expanded)?;
    finalize_png(source, &encoded)
}

fn encode_png(img: &image::DynamicImage) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        &mut buf,
        CompressionType::Best,
        FilterType::NoFilter,
    );
    match img {
        image::DynamicImage::ImageLuma8(i) => encoder.write_image(
            i.as_raw(),
            i.width(),
            i.height(),
            ExtendedColorType::L8,
        )?,
        image::DynamicImage::ImageLumaA8(i) => encoder.write_image(
            i.as_raw(),
            i.width(),
            i.height(),
            ExtendedColorType::La8,
        )?,
        image::DynamicImage::ImageRgb8(i) => encoder.write_image(
            i.as_raw(),
            i.width(),
            i.height(),
            ExtendedColorType::Rgb8,
        )?,
        image::DynamicImage::ImageRgba8(i) => encoder.write_image(
            i.as_raw(),
            i.width(),
            i.height(),
            ExtendedColorType::Rgba8,
        )?,
        other => {
            let rgba = other.to_rgba8();
            encoder.write_image(
                rgba.as_raw(),
                rgba.width(),
                rgba.height(),
                ExtendedColorType::Rgba8,
            )?;
        }
    }
    Ok(buf)
}

/// TIFF: expand and re-encode (limited IFD tag preservation).
fn process_tiff(source: &[u8], border: u32, color: rgb::RGB8) -> Result<Vec<u8>> {
    let img = image::load_from_memory(source).context("decode TIFF")?;
    let expanded = border::expand(img, border, color)?;
    encode_tiff(&expanded)
}

fn encode_tiff(img: &image::DynamicImage) -> Result<Vec<u8>> {
    let mut buf = Cursor::new(Vec::new());
    let enc = TiffEncoder::new(&mut buf);
    match img {
        image::DynamicImage::ImageRgb8(i) => {
            enc.write_image(i.as_raw(), i.width(), i.height(), ExtendedColorType::Rgb8)?;
        }
        image::DynamicImage::ImageRgba8(i) => {
            enc.write_image(i.as_raw(), i.width(), i.height(), ExtendedColorType::Rgba8)?;
        }
        image::DynamicImage::ImageLuma8(i) => {
            enc.write_image(i.as_raw(), i.width(), i.height(), ExtendedColorType::L8)?;
        }
        other => {
            let rgb = other.to_rgb8();
            enc.write_image(
                rgb.as_raw(),
                rgb.width(),
                rgb.height(),
                ExtendedColorType::Rgb8,
            )?;
        }
    }
    Ok(buf.into_inner())
}

fn detect_format(path: &PathBuf) -> Result<ImageFormat> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .with_context(|| format!("no extension on {}", path.display()))?;

    match ext.as_str() {
        "jpg" | "jpeg" | "jfif" => Ok(ImageFormat::Jpeg),
        "png" => Ok(ImageFormat::Png),
        "tif" | "tiff" => Ok(ImageFormat::Tiff),
        "jxl" | "jpxl" | "jpx" => Ok(ImageFormat::Jxl),
        other => bail!("unsupported format .{other} (supported: jpg, jpeg, png, tiff, jxl/jpxl)"),
    }
}
