mod cli;

use std::path::PathBuf;

use clap::Parser;

use cli::{Cli, Commands};
use wod::{run_add_wod_from_file, run_add_workout, run_base};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Instead of this, rewrite the part from cli to call this function
        Some(Commands::Add(add_command)) => {
            // The add command "wod add 'workout' -f 'date-filename.md' "
            let filename = PathBuf::from(add_command.filename.to_string());
            let _ = run_add_workout(filename, &add_command.workout);
            println!("Added workout to file: {}", add_command.filename);
        }
        None => {
            // The base command "wod 'date-filename.md'"
            let filename = PathBuf::from(cli.filename.to_string());
            if cli.wodfile.is_some() {
                // Check/Parse the filename
                let wodfile = PathBuf::from(cli.wodfile.unwrap());
                println!("Creating file from WOD file: {}", wodfile.display());
                let _ = run_add_wod_from_file(filename, wodfile);
            } else {
                println!("Creating file: {}", filename.display());
                let _ = run_base(filename, &cli.force);
            }
        }
    }
}
