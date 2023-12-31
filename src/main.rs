#![feature(absolute_path)]
pub mod commands;
pub mod utils;

use anyhow::Result;
use clap::Parser;
use rexiv2::Metadata;
use utils::append_to_path;

use commands::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    for src in cli.input.iter() {
        assert!(src.is_absolute());
        let dst = match cli.output {
            Some(ref e) => e.clone(),
            None => append_to_path(src.clone(), ".jpg"),
        };

        println!("Processing image {:?}", src);
        let metadata = Metadata::new_from_path(&src)?;

        if let Some(previews) = metadata.get_preview_images() {
            // Get preview with highest resolution
            let preview = previews
                .iter()
                .max_by_key(|e| e.get_size())
                .expect("previews is not supposed to be empty");
            preview
                .save_to_file(&dst)
                .expect("failed to save preview to file");
            println!("Stored preview to {:?}", dst);

            let ext = preview.get_extension().expect("failed to get extension");
            metadata
                .save_to_file(append_to_path(dst.clone(), &ext))
                .expect("failed to save metadata to file");
            println!("Stored metadata to {:?}", dst);
        } else {
            println!("Image {:?} contains no embedded JPG", src);
        }
    }

    Ok(())
}
