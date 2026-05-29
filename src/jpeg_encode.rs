//! JPEG pipeline: decode with `jpeg-decoder`, expand pixels in memory, encode with
//! source quantization tables / subsampling, then splice APP/COM/DQT from the source file.

use std::io::Cursor;

use anyhow::{Context, Result};
use img_parts::jpeg::{markers, Jpeg};
use img_parts::Bytes;
use jpeg_decoder::{Decoder, ImageInfo, PixelFormat};
use jpeg_encoder::{ColorType, Encoder, QuantizationTableType, SamplingFactor};
use rgb::RGB8;

use crate::metadata::splice_jpeg;

/// Full JPEG workflow for one file buffer (used before atomic write in `formats`).
pub fn process_jpeg(source: &[u8], border: u32, color: RGB8) -> Result<Vec<u8>> {
    let (pixels, info) = decode_jpeg(source)?;
    let (new_w, new_h, expanded) = expand_pixels(&pixels, &info, border, color)?;
    let encoded = encode_scan(&expanded, new_w, new_h, &info, source)?;
    splice_jpeg(source, &encoded)
}

fn decode_jpeg(source: &[u8]) -> Result<(Vec<u8>, ImageInfo)> {
    let mut dec = Decoder::new(Cursor::new(source));
    dec.read_info().context("read JPEG info")?;
    let info = dec.info().context("missing JPEG info")?.clone();
    let pixels = dec.decode().context("decode JPEG pixels")?;
    Ok((pixels, info))
}

fn expand_pixels(
    pixels: &[u8],
    info: &ImageInfo,
    border: u32,
    color: RGB8,
) -> Result<(u16, u16, Vec<u8>)> {
    let w = info.width as usize;
    let h = info.height as usize;
    let bpp = info.pixel_format.pixel_bytes();
    let b = border as usize;
    let nw = w + 2 * b;
    let nh = h + 2 * b;
    let mut out = vec![0u8; nw * nh * bpp];

    let fill = match info.pixel_format {
        PixelFormat::L8 | PixelFormat::L16 => vec![color.r],
        PixelFormat::RGB24 => vec![color.r, color.g, color.b],
        PixelFormat::CMYK32 => vec![0u8, 0u8, 0u8, 0u8],
    };

    for y in 0..nh {
        for x in 0..nw {
            let px = (y * nw + x) * bpp;
            for (i, &v) in fill.iter().enumerate().take(bpp) {
                out[px + i] = v;
            }
        }
    }

    for y in 0..h {
        let src_row = y * w * bpp;
        let dst_row = ((y + b) * nw + b) * bpp;
        out[dst_row..dst_row + w * bpp].copy_from_slice(&pixels[src_row..src_row + w * bpp]);
    }

    Ok((nw as u16, nh as u16, out))
}

fn encode_scan(
    pixels: &[u8],
    width: u16,
    height: u16,
    info: &ImageInfo,
    source: &[u8],
) -> Result<Vec<u8>> {
    let (luma_q, chroma_q) = quantization_tables_from_source(source)?;
    let sampling = sampling_from_source(source)?;

    let color_type = match info.pixel_format {
        PixelFormat::L8 => ColorType::Luma,
        PixelFormat::L16 => ColorType::Luma,
        PixelFormat::RGB24 => ColorType::Rgb,
        PixelFormat::CMYK32 => ColorType::Cmyk,
    };

    let mut buf = Vec::new();
    let mut enc = Encoder::new(&mut buf, 100);
    enc.set_optimized_huffman_tables(false);
    enc.set_sampling_factor(sampling);
    enc.set_quantization_tables(luma_q, chroma_q);
    enc.encode(pixels, width, height, color_type)
        .context("encode JPEG scan")?;
    Ok(buf)
}

fn quantization_tables_from_source(
    source: &[u8],
) -> Result<(QuantizationTableType, QuantizationTableType)> {
    let jpeg = Jpeg::from_bytes(Bytes::from(source.to_vec()))?;
    let mut tables: Vec<[u8; 64]> = Vec::new();

    for seg in jpeg.segments() {
        if seg.marker() != markers::DQT {
            continue;
        }
        let data = seg.contents();
        let mut i = 0usize;
        while i + 65 <= data.len() {
            tables.push(array_from_dqt(&data[i..i + 65])?);
            i += 65;
        }
        if !tables.is_empty() {
            break;
        }
    }

    let luma = tables
        .first()
        .copied()
        .map(qtable_from_bytes)
        .unwrap_or_else(default_luma_qtable);
    let chroma = tables
        .get(1)
        .copied()
        .map(qtable_from_bytes)
        .unwrap_or_else(|| luma.clone());
    Ok((luma, chroma))
}

fn qtable_from_bytes(bytes: [u8; 64]) -> QuantizationTableType {
    let mut table = [0u16; 64];
    for (i, &b) in bytes.iter().enumerate() {
        table[i] = b as u16;
    }
    QuantizationTableType::Custom(Box::new(table))
}

fn array_from_dqt(chunk: &[u8]) -> Result<[u8; 64]> {
    if chunk.len() < 65 {
        anyhow::bail!("short DQT table");
    }
    let mut out = [0u8; 64];
    out.copy_from_slice(&chunk[1..65]);
    Ok(out)
}

fn default_luma_qtable() -> QuantizationTableType {
    qtable_from_bytes([
        16, 11, 10, 16, 24, 40, 51, 61, 12, 12, 14, 19, 26, 58, 60, 55, 14, 13, 16, 24, 40, 57,
        69, 56, 14, 17, 22, 29, 51, 87, 80, 62, 18, 22, 37, 56, 68, 109, 103, 77, 24, 35, 55, 64,
        81, 104, 113, 92, 49, 64, 78, 87, 103, 121, 120, 101, 72, 92, 95, 98, 112, 100, 103, 99,
    ])
}

fn sampling_from_source(source: &[u8]) -> Result<SamplingFactor> {
    let jpeg = Jpeg::from_bytes(Bytes::from(source.to_vec()))?;
    let sof = jpeg
        .segments()
        .iter()
        .find(|s| is_sof(s.marker()))
        .context("source JPEG missing SOF")?;
    let data = sof.contents();
    if data.len() < 8 {
        return Ok(SamplingFactor::F_1_1);
    }
    let nf = data[5] as usize;
    if nf < 1 || data.len() < 6 + nf * 3 {
        return Ok(SamplingFactor::F_1_1);
    }
    let hv_y = data[7];
    let h0 = (hv_y >> 4) & 0x0f;
    let v0 = hv_y & 0x0f;
    let (h1, v1) = if nf >= 2 {
        let hv = data[10];
        ((hv >> 4) & 0x0f, hv & 0x0f)
    } else {
        (1, 1)
    };
    Ok(if h0 == 2 && v0 == 2 && h1 <= 1 && v1 <= 1 {
        SamplingFactor::F_2_2
    } else if h0 == 2 && v0 == 1 && h1 <= 1 && v1 <= 1 {
        SamplingFactor::F_2_1
    } else {
        SamplingFactor::F_1_1
    })
}

fn is_sof(marker: u8) -> bool {
    matches!(
        marker,
        markers::SOF0
            | markers::SOF1
            | markers::SOF2
            | markers::SOF3
            | markers::SOF5
            | markers::SOF6
            | markers::SOF7
            | markers::SOF9
            | markers::SOF10
            | markers::SOF11
            | markers::SOF13
            | markers::SOF14
            | markers::SOF15
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn center_pixels_match_decoded_buffer() -> Result<()> {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bak = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains("Painting") && n.ends_with(".bak.jpg"))
                    .unwrap_or(false)
            })
            .unwrap_or_else(|| dir.join("test.jpg"));

        if !bak.exists() {
            return Ok(());
        }

        let source = std::fs::read(&bak)?;
        let (orig_px, info) = decode_jpeg(&source)?;
        let w = info.width;
        let h = info.height;

        // Baseline: same-size re-encode loss (unavoidable for lossy JPEG).
        let same_size = encode_scan(&orig_px, w, h, &info, &source)?;
        let roundtrip = splice_jpeg(&source, &same_size)?;
        let (rt_px, _) = decode_jpeg(&roundtrip)?;
        let bpp = info.pixel_format.pixel_bytes();
        let mut max_rt = 0u8;
        for i in 0..(orig_px.len() / bpp) {
            for c in 0..bpp {
                let o = orig_px[i * bpp + c];
                let r = rt_px[i * bpp + c];
                max_rt = max_rt.max(o.abs_diff(r));
            }
        }

        let out = process_jpeg(&source, 40, RGB8::new(255, 255, 255))?;
        let (out_px, out_info) = decode_jpeg(&out)?;
        let b = 40usize;
        let wi = w as usize;
        let hi = h as usize;
        let nw = out_info.width as usize;

        let mut max_delta = 0u8;
        for y in 0..hi {
            for x in 0..wi {
                for c in 0..bpp {
                    let o = orig_px[(y * wi + x) * bpp + c];
                    let n = out_px[((y + b) * nw + (x + b)) * bpp + c];
                    max_delta = max_delta.max(o.abs_diff(n));
                }
            }
        }

        assert!(
            max_delta <= max_rt.saturating_add(2),
            "border path added more loss than same-size roundtrip (border={max_delta}, roundtrip={max_rt})"
        );
        Ok(())
    }

    #[test]
    fn metadata_app_segments_preserved() -> Result<()> {
        use crate::metadata::metadata_segment_bytes;

        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let bak = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .find(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .map(|n| n.contains("Painting") && n.ends_with(".bak.jpg"))
                    .unwrap_or(false)
            })
            .unwrap_or_else(|| dir.join("test.jpg"));

        if !bak.exists() {
            return Ok(());
        }

        let source = std::fs::read(&bak)?;
        let out = process_jpeg(&source, 40, RGB8::new(255, 255, 255))?;
        let src_meta = metadata_segment_bytes(&source)?;
        let out_meta = metadata_segment_bytes(&out)?;

        assert_eq!(
            src_meta, out_meta,
            "APP/COM segment payloads must be byte-identical"
        );
        Ok(())
    }
}
