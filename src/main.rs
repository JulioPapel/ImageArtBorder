//! ImageArtBorder — CLI entry point.
//!
//! Author: Júlio Papel <info@juliopapel.pt>
//! License: MIT
//!
//! Expands image files with a solid-color border while preserving metadata
//! and copying the original pixel region without resampling.

mod border;
mod border_calc;
mod cli;
mod formats;
mod jpeg_encode;
mod metadata;

use anyhow::Result;
use clap::Parser;
use cli::Args;

fn main() -> Result<()> {
    let args = Args::parse();
    formats::process(&args)?;
    Ok(())
}
