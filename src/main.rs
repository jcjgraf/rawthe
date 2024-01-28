#![feature(absolute_path)]
pub mod commands;
pub mod error;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use rexiv2::{Metadata, PreviewImage};

use commands::Cli;
use error::General;

/// Get preview with largest resolution.
fn get_largest_preview(metadata: &Metadata) -> Result<PreviewImage, General> {
    let previews: Vec<PreviewImage> = metadata.get_preview_images().ok_or(General::NoThumbnail)?;

    let largest_preview: PreviewImage = previews
        .into_iter()
        .max_by_key(|e| e.get_size())
        .expect("previews is not supposed to be empty");

    Ok(largest_preview)
}

/// Extract thumbnail of `src` and store to `output_dir`.
fn process_image(src: &PathBuf, output_dir: &Option<PathBuf>) -> Result<PathBuf, General> {
    assert!(src.is_absolute());
    let metadata = Metadata::new_from_path(src).map_err(|e| General::LoadMetadata(e))?;
    let preview = get_largest_preview(&metadata)?;

    let mut dst = match output_dir {
        Some(e) => e.to_owned(),
        None => src
            .parent()
            .expect("image is supposed to have a parent")
            .to_owned(),
    };
    assert!(dst.is_dir());

    // Set filename to name of source, without extension as `save_to_file` will add the appropriate extension
    dst.push(
        src.file_stem()
            .expect("image is supposed to have a filename"),
    );

    preview
        .save_to_file(&dst)
        .map_err(|e| General::StoreThumbnail(e))?;

    // Add correct extension
    dst.set_extension(
        preview
            .get_extension()
            .map_err(|e| General::GetExtension(e))?
            .strip_prefix(".")
            .expect("extension is supposed to start with ."),
    );
    assert!(dst.is_file());

    metadata
        .save_to_file(&dst)
        .map_err(|e| General::StoreMetadata(e))?;

    Ok(dst)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for src in cli.input_images.iter() {
        match process_image(src, &cli.output_dir) {
            Ok(dst) => println!("Stored thumbnail to {:?}", dst),
            Err(e) => {
                println!("Error while processing image {:?}: {}", src, e);
            }
        }
    }

    Ok(())
}
