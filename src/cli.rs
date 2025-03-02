use clap::{Parser, Subcommand};

use std::fs;
use std::io;
use std::path::PathBuf;
use wod::default_filename;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(value_name = "FILENAME", default_value_t = default_filename())]
    pub filename: String,

    #[command(subcommand)]
    pub command: Option<Commands>,

    // TODO: Add option to overwrite the file
    #[arg(short, long, default_value = "false")]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds a new workout
    Add(AddCommand),
}

#[derive(Parser, Debug)]
pub struct AddCommand {
    /// Optional name to operate on
    #[arg(short, long, value_name = "FILENAME", default_value_t = default_filename())]
    pub filename: String,

    /// The movement to add for the moment, should be a single piece of a workout
    #[arg(value_name = "air squat, 5x5")]
    pub workout: String,
}
