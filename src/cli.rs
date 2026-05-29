//! Command-line interface: border percent, color, and input path.

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use rgb::RGB8;

/// Command-line arguments for `ImageArtBorder.exe`.
#[derive(Parser, Debug)]
#[command(
    name = "ImageArtBorder",
    author = "Júlio Papel <info@juliopapel.pt>",
    version,
    about = "Expand an image with a solid-color border; preserves metadata and original pixels where possible",
    disable_help_subcommand = true
)]
pub struct Args {
    /// Border as a percentage increase of the image diagonal (default: 6.0).
    /// Example: 6.0 makes the new diagonal 6% longer than the original.
    #[arg(short = 'b', long = "border", default_value_t = 6.0)]
    pub border_percent: f64,

    /// Border color as #RRGGBB or #AARRGGBB (default: #FFFFFF)
    #[arg(short = 'c', long = "color", default_value = "#FFFFFF")]
    pub color: String,

    /// Image file to process (optional if path is given as the last argument)
    #[arg(short = 'f', long = "file")]
    pub file: Option<PathBuf>,

    /// Image path (must be the last argument when -f is not used)
    #[arg(value_name = "IMAGE_PATH")]
    pub image_path: Option<PathBuf>,
}

impl Args {
    /// Resolved input path: `-f` wins over the trailing positional argument.
    pub fn image_path(&self) -> Result<PathBuf> {
        if let Some(p) = &self.file {
            return Ok(p.clone());
        }
        if let Some(p) = &self.image_path {
            return Ok(p.clone());
        }
        bail!("no image path: use -f <path> or put the image path as the last argument");
    }

    /// Validate border percentage from the CLI.
    pub fn validate_border_percent(&self) -> Result<()> {
        if self.border_percent < 0.0 {
            bail!("border percent must be >= 0, got {}", self.border_percent);
        }
        if self.border_percent > 100.0 {
            bail!(
                "border percent above 100 is unusual; got {}. Use a value like 6 for 6%.",
                self.border_percent
            );
        }
        Ok(())
    }
}

/// Parse `#RRGGBB` or `#AARRGGBB` into an RGB8 color (alpha is ignored).
pub fn parse_color(s: &str) -> Result<RGB8> {
    let hex = s.trim().trim_start_matches('#');
    let bytes = match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).context("invalid color hex")?;
            let g = u8::from_str_radix(&hex[2..4], 16).context("invalid color hex")?;
            let b = u8::from_str_radix(&hex[4..6], 16).context("invalid color hex")?;
            (r, g, b)
        }
        8 => {
            let _a = u8::from_str_radix(&hex[0..2], 16).context("invalid color hex")?;
            let r = u8::from_str_radix(&hex[2..4], 16).context("invalid color hex")?;
            let g = u8::from_str_radix(&hex[4..6], 16).context("invalid color hex")?;
            let b = u8::from_str_radix(&hex[6..8], 16).context("invalid color hex")?;
            (r, g, b)
        }
        _ => bail!("color must be #RRGGBB or #AARRGGBB, got {s}"),
    };
    Ok(RGB8 {
        r: bytes.0,
        g: bytes.1,
        b: bytes.2,
    })
}
