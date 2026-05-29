//! PNG chunk preservation: copy all non-image chunks from the source file.

use anyhow::Result;
use img_parts::png::Png;
use img_parts::Bytes;

/// Preserve every source chunk except IHDR/IDAT/IEND (replaced by the encoded image data).
pub fn finalize_png(source: &[u8], encoded: &[u8]) -> Result<Vec<u8>> {
    let src = Png::from_bytes(Bytes::from(source.to_vec()))?;
    let mut out = Png::from_bytes(Bytes::from(encoded.to_vec()))?;

    let new_ihdr = out
        .chunks()
        .iter()
        .find(|c| c.kind() == *b"IHDR")
        .ok_or_else(|| anyhow::anyhow!("encoded PNG missing IHDR"))?
        .clone();

    let new_idat: Vec<_> = out
        .chunks()
        .iter()
        .filter(|c| c.kind() == *b"IDAT")
        .cloned()
        .collect();

    let mut out_chunks = vec![new_ihdr];

    for chunk in src.chunks() {
        let k = chunk.kind();
        if k == *b"IHDR" || k == *b"IDAT" || k == *b"IEND" {
            continue;
        }
        out_chunks.push(chunk.clone());
    }

    out_chunks.extend(new_idat);
    if let Some(iend) = out.chunks().iter().find(|c| c.kind() == *b"IEND") {
        out_chunks.push(iend.clone());
    }

    *out.chunks_mut() = out_chunks;

    let mut buf = Vec::new();
    out.encoder().write_to(&mut buf)?;
    Ok(buf)
}
