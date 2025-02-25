mod cli;
mod utils;

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use clap::Parser;

use cli::{Cli, Commands};
use wod::run_add_workout;


// Move the run_ functions to lib.rs
fn run_base(folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: This command should create the folder in the current directory
    // if it doesn't exist, and in the future, write the yaml content expected from Hugo.

    // The base command creates the folder
    let folder_name = format!("wod-{}", &folder);
    let dir_path = Path::new(&folder_name);

    // Check if the directory already exists
    if !dir_path.exists() {
        // Create the directory
        fs::create_dir(dir_path)?;
    }
    let filename = dir_path.join(utils::default_filename());
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&filename)?;
    // TODO: Create a template string for the file content
    file.write_all(
        // TODO: Create a template string for the file content
        // Use format!(...)
        b"Hello, world!",
    )?;

    Ok(())
}


fn main() {
    let cli = Cli::parse();

    println!("The input is: {}", cli.folder);
    // let _ = run_base(&cli.folder);
    // For the moment create the folder in the current dir, and can only pass the folder name, not the full path (say draft/wod-01-01-2021)

    match &cli.command {
        // Instead of this, rewrite the part from cli to call this function
        Some(Commands::Add(add_command)) => {
            println!("Adding workout to file: {}", add_command.filename);

            let _ = run_add_workout(&add_command.filename, &add_command.workout);
            println!("DONE");
        }
        None => {} // Do nothing
    }
}
