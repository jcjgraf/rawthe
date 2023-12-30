use clap::Parser;
use std::path::{self, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[arg(required=true, num_args=1.., value_parser = validate_path)]
    pub input: Vec<PathBuf>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}

/// Ensures that a provided path is an existing file
fn validate_path(image: &str) -> Result<PathBuf, String> {
    let image: PathBuf = path::absolute(image).expect("Failed to get absolute path");
    if !image.exists() {
        Err(format!("File {} does not exist.", image.to_str().unwrap()))
    } else if !image.is_file() {
        Err(format!(
            "{} does not appear to be a file.",
            image.to_str().unwrap()
        ))
    } else {
        Ok(image)
    }
}
