//! Border width from a percentage increase of the image diagonal.

/// Compute uniform border width (pixels per side) so the new diagonal is
/// `percent` percent larger than the original.
///
/// Given width `w`, height `h`, diagonal `d = sqrt(w² + h²)`, find `b` such that:
/// `sqrt((w + 2b)² + (h + 2b)²) = d × (1 + percent / 100)`
pub fn pixels_from_diagonal_percent(width: u32, height: u32, percent: f64) -> u32 {
    if percent <= 0.0 || width == 0 || height == 0 {
        return 0;
    }

    let w = width as f64;
    let h = height as f64;
    let d_sq = w * w + h * h;
    let scale = 1.0 + percent / 100.0;
    let target_d_sq = d_sq * scale * scale;

    // (w + 2b)² + (h + 2b)² = target_d_sq
    // 8b² + 4(w + h)b + d_sq - target_d_sq = 0
    let a = 8.0;
    let b_lin = 4.0 * (w + h);
    let c = d_sq - target_d_sq;
    let discriminant = b_lin * b_lin - 4.0 * a * c;

    if discriminant < 0.0 {
        return 0;
    }

    let b = (-b_lin + discriminant.sqrt()) / (2.0 * a);
    b.round().clamp(0.0, u32::MAX as f64) as u32
}

/// Original and new diagonal lengths for logging or tests.
pub fn diagonals(width: u32, height: u32, border_px: u32) -> (f64, f64) {
    let w = width as f64;
    let h = height as f64;
    let old = (w * w + h * h).sqrt();
    let bw = w + 2.0 * border_px as f64;
    let bh = h + 2.0 * border_px as f64;
    let new = (bw * bw + bh * bh).sqrt();
    (old, new)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_six_percent_diagonal_increase() {
        let b = pixels_from_diagonal_percent(100, 100, 6.0);
        let (old, new) = diagonals(100, 100, b);
        let ratio = new / old;
        assert!((ratio - 1.06).abs() < 0.001, "ratio={ratio}, b={b}");
    }

    #[test]
    fn zero_percent_is_zero_border() {
        assert_eq!(pixels_from_diagonal_percent(1000, 500, 0.0), 0);
    }
}
