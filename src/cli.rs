use clap::{Parser, Subcommand};

use std::fs;
use std::io;
use std::path::Path;

use crate::utils::{default_filename, today};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    #[arg(short, long, value_name = "DATE", default_value_t = today())]
    pub folder: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Adds a new workout
    Add(AddCommand),
}

#[derive(Parser, Debug)]
pub struct AddCommand {
    /// Optional name to operate on
    #[arg(short, long, value_name = "DATE", default_value_t = default_filename())]
    pub filename: String,

    /// The movement to add for the moment, should be a single piece of a workout
    #[arg(value_name = "air squat, 5x5")]
    pub workout: String,
}
