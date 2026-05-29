//! JPEG marker-level splice: preserve APP/COM/DQT bytes from the source file.

use anyhow::{bail, Result};
use img_parts::jpeg::{markers, Jpeg, JpegSegment};
use img_parts::Bytes;

/// Rebuild JPEG:
/// - APP/COM and DQT segments from `source` (byte-identical)
/// - DHT, SOF, SOS, entropy from `encoded` (Huffman must match scan data)
pub fn splice_jpeg(source: &[u8], encoded: &[u8]) -> Result<Vec<u8>> {
    let src = Jpeg::from_bytes(Bytes::from(source.to_vec()))?;
    let enc = Jpeg::from_bytes(Bytes::from(encoded.to_vec()))?;

    let mut prefix: Vec<JpegSegment> = Vec::new();
    for seg in src.segments() {
        let m = seg.marker();
        if is_metadata_marker(m) || m == markers::DQT {
            prefix.push(seg.clone());
        }
    }

    let image_start = enc
        .segments()
        .iter()
        .position(|s| s.marker() == markers::DHT || is_sof(s.marker()))
        .ok_or_else(|| anyhow::anyhow!("encoded JPEG missing DHT/SOF"))?;

    let mut combined = prefix;
    combined.extend(enc.segments()[image_start..].iter().cloned());

    if !combined.iter().any(|s| s.marker() == markers::SOS) {
        bail!("spliced JPEG missing SOS");
    }

    let mut out = Jpeg::from_bytes(Bytes::from(encoded.to_vec()))?;
    *out.segments_mut() = combined;

    let mut buf = Vec::new();
    out.encoder().write_to(&mut buf)?;
    Ok(buf)
}

/// Collect raw APP/COM segment payloads from a JPEG (marker byte + contents, no entropy).
pub fn metadata_segment_bytes(jpeg: &[u8]) -> Result<Vec<(u8, Vec<u8>)>> {
    let j = Jpeg::from_bytes(Bytes::from(jpeg.to_vec()))?;
    Ok(j.segments()
        .iter()
        .filter(|s| is_metadata_marker(s.marker()))
        .map(|s| (s.marker(), s.contents().as_ref().to_vec()))
        .collect())
}

fn is_metadata_marker(marker: u8) -> bool {
    matches!(
        marker,
        markers::APP0
            | markers::APP1
            | markers::APP2
            | markers::APP3
            | markers::APP4
            | markers::APP5
            | markers::APP6
            | markers::APP7
            | markers::APP8
            | markers::APP9
            | markers::APP10
            | markers::APP11
            | markers::APP12
            | markers::APP13
            | markers::APP14
            | markers::APP15
            | markers::COM
    )
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
