//! Canvas expansion: fill a larger buffer with the border color and copy the
//! source image into the center without scaling or filtering.

use anyhow::{bail, Result};
use image::{
    DynamicImage, GenericImage, GenericImageView, ImageBuffer, Luma, LumaA, Pixel, Rgba, Rgb,
};
use rgb::RGB8;

/// Expand the image by `border` pixels on each side.
///
/// The original pixel rectangle is copied with `copy_from` (no resampling).
/// Only the new outer rows/columns are filled with `color`.
pub fn expand(img: DynamicImage, border: u32, color: RGB8) -> Result<DynamicImage> {
    if border == 0 {
        return Ok(img);
    }

    let (w, h) = img.dimensions();
    let new_w = w.saturating_add(border.saturating_mul(2));
    let new_h = h.saturating_add(border.saturating_mul(2));

    let fill = Rgb([color.r, color.g, color.b]);
    let fill_a = Rgba([color.r, color.g, color.b, 255]);

    let out = match img {
        DynamicImage::ImageLuma8(ref inner) => {
            let mut canvas: ImageBuffer<Luma<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(new_w, new_h, Luma([color.r]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageLuma8(canvas)
        }
        DynamicImage::ImageLumaA8(ref inner) => {
            let mut canvas: ImageBuffer<LumaA<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(new_w, new_h, LumaA([color.r, 255]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageLumaA8(canvas)
        }
        DynamicImage::ImageRgb8(ref inner) => {
            let mut canvas: ImageBuffer<Rgb<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(new_w, new_h, fill);
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageRgb8(canvas)
        }
        DynamicImage::ImageRgba8(ref inner) => {
            let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(new_w, new_h, fill_a);
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageRgba8(canvas)
        }
        DynamicImage::ImageLuma16(ref inner) => {
            let v = ((color.r as u16) << 8) | (color.r as u16);
            let mut canvas: ImageBuffer<Luma<u16>, Vec<u16>> =
                ImageBuffer::from_pixel(new_w, new_h, Luma([v]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageLuma16(canvas)
        }
        DynamicImage::ImageLumaA16(ref inner) => {
            let v = ((color.r as u16) << 8) | (color.r as u16);
            let mut canvas: ImageBuffer<LumaA<u16>, Vec<u16>> =
                ImageBuffer::from_pixel(new_w, new_h, LumaA([v, 65535]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageLumaA16(canvas)
        }
        DynamicImage::ImageRgb16(ref inner) => {
            let r = ((color.r as u16) << 8) | (color.r as u16);
            let g = ((color.g as u16) << 8) | (color.g as u16);
            let b = ((color.b as u16) << 8) | (color.b as u16);
            let mut canvas: ImageBuffer<image::Rgb<u16>, Vec<u16>> =
                ImageBuffer::from_pixel(new_w, new_h, image::Rgb([r, g, b]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageRgb16(canvas)
        }
        DynamicImage::ImageRgba16(ref inner) => {
            let r = ((color.r as u16) << 8) | (color.r as u16);
            let g = ((color.g as u16) << 8) | (color.g as u16);
            let b = ((color.b as u16) << 8) | (color.b as u16);
            let mut canvas: ImageBuffer<Rgba<u16>, Vec<u16>> =
                ImageBuffer::from_pixel(new_w, new_h, Rgba([r, g, b, 65535]));
            blit(inner, &mut canvas, border, border)?;
            DynamicImage::ImageRgba16(canvas)
        }
        other => {
            // Uncommon color models: one conversion to RGBA, then exact blit.
            let rgba = other.to_rgba8();
            let mut canvas: ImageBuffer<Rgba<u8>, Vec<u8>> =
                ImageBuffer::from_pixel(new_w, new_h, fill_a);
            blit(&rgba, &mut canvas, border, border)?;
            DynamicImage::ImageRgba8(canvas)
        }
    };

    Ok(out)
}

/// Copy `src` into `dst` at offset `(x, y)` with no blending.
fn blit<P, S, D>(src: &ImageBuffer<P, S>, dst: &mut ImageBuffer<P, D>, x: u32, y: u32) -> Result<()>
where
    P: Pixel,
    S: std::ops::Deref<Target = [P::Subpixel]>,
    D: std::ops::DerefMut<Target = [P::Subpixel]>,
    ImageBuffer<P, S>: GenericImageView<Pixel = P>,
    ImageBuffer<P, D>: GenericImage<Pixel = P>,
{
    if x.saturating_add(src.width()) > dst.width()
        || y.saturating_add(src.height()) > dst.height()
    {
        bail!("internal error: border blit out of bounds");
    }
    dst.copy_from(src, x, y)?;
    Ok(())
}
