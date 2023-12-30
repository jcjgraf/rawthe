#![feature(absolute_path)]
pub mod commands;
pub mod utils;

use clap::Parser;
use anyhow::Result;
use utils::append_to_path;

use commands::Cli;

fn main() -> Result<()>{
    let cli = Cli::parse();

    for src in cli.input.iter() {
        assert!(src.is_absolute());
        let dst = match cli.output {
            Some(ref e) => e.clone(),
            None => {
                append_to_path(src.clone(), ".jpg")
            }
        };
        println!("Processing image {:?}", src);
        println!("Storing jpg to   {:?}", dst);
    }

    Ok(())
}
