use clap::Parser;
use std::path::{self, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[arg(required=true, num_args=1.., value_parser = validate_file)]
    pub input_images: Vec<PathBuf>,

    #[arg(short, long, value_parser=validate_dir)]
    pub output_dir: Option<PathBuf>,
}

/// Ensures that a provided path is an existing file
fn validate_file(file: &str) -> Result<PathBuf, String> {
    let file: PathBuf = path::absolute(file).expect("Failed to get absolute path");
    if !file.exists() {
        Err(format!("File {} does not exist.", file.to_str().unwrap()))
    } else if !file.is_file() {
        Err(format!(
            "{} does not appear to be a file.",
            file.to_str().unwrap()
        ))
    } else {
        Ok(file)
    }
}

/// Ensures that a provided path is an existing directory
fn validate_dir(dir: &str) -> Result<PathBuf, String> {
    let dir: PathBuf = path::absolute(dir).expect("Failed to get absolute path");
    if !dir.exists() {
        Err(format!("File {} does not exist.", dir.to_str().unwrap()))
    } else if !dir.is_dir() {
        Err(format!(
            "{} does not appear to be a directory.",
            dir.to_str().unwrap()
        ))
    } else {
        println!("path {:?} filename {:?}", dir, dir.file_name());
        Ok(dir)
    }
}
