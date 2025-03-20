mod cli;

use std::path::PathBuf;

use clap::Parser;

use cli::{Cli, Commands};
use wod::{run_add_wod_from_file, run_add_workout, run_base, run_create_list_movements};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add(add_command)) => {
            // The add command "wod add 'workout' -f 'date-filename.md' "
            let filename = PathBuf::from(add_command.filename.to_string());
            let _ = run_add_workout(
                filename,
                &add_command.workout,
                add_command.comments.clone(),
                add_command.name.clone(),
            );
            println!("Added workout to file: {}", add_command.filename);
        }
        Some(Commands::List(list_command)) => {
            let movement_list = run_create_list_movements(list_command.page);
            println!("{}", movement_list);
        }
        None => {
            // The base command "wod 'date-filename.md'"
            let filename = PathBuf::from(cli.filename.to_string());
            if cli.wodfile.is_some() {
                // Check/Parse the filename
                let wodfile = PathBuf::from(cli.wodfile.unwrap());
                let _ = run_add_wod_from_file(filename, wodfile, cli.file_date, cli.languages);
            } else {
                println!("Creating file: {}", filename.display());
                let _ = run_base(filename, &cli.force, cli.file_date, cli.languages);
            }
        }
    }
}
