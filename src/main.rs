#![feature(absolute_path)]
pub mod commands;

use anyhow::Result;
use clap::Parser;
use rexiv2::Metadata;

use commands::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    for src in cli.input_images.iter() {
        assert!(src.is_absolute());

        println!("Processing image {:?}", src);
        let metadata = Metadata::new_from_path(&src)?;

        if let Some(previews) = metadata.get_preview_images() {
            // Get preview with highest resolution
            let preview = previews
                .iter()
                .max_by_key(|e| e.get_size())
                .expect("previews is not supposed to be empty");

            let mut dst = match cli.output_dir {
                Some(ref e) => e.to_owned(),
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
                .expect("failed to save preview to file");

            dst.set_extension(
                preview
                    .get_extension()
                    .expect("failed to get preview extension")
                    .strip_prefix(".")
                    .expect("extension is supposed to start with ."),
            );
            assert!(dst.is_file());

            metadata
                .save_to_file(&dst)
                .expect("failed to save metadata to file");
            println!("Stored preview to {:?}", dst);
        } else {
            println!("Image {:?} contains no embedded JPG", src);
        }
    }

    Ok(())
}
