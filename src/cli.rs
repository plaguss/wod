use clap::{Parser, Subcommand};

use wod::default_filename;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional name to work with.
    #[arg(default_value_t = default_filename())]
    pub filename: String,

    /// A path pointing to a file with a list of workouts to add.
    /// Each line in the file should be a workout as you would pass
    /// to `wod add <workout>`.
    #[arg(short, long)]
    pub wodfile: Option<String>,

    /// Subcommands
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Whether to force overwriting an existing file, defaults to false.
    #[arg(short, long, default_value = "false")]
    pub force: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Command to add a new workout to a file.
    Add(AddCommand),
    /// Command to list the movements along with an explanatory video.
    List(ListCommand),
}

#[derive(Parser, Debug)]
pub struct AddCommand {
    /// The filename to add the workout to. By default will use the same used with the `wod` command.
    #[arg(short, long, default_value_t = default_filename())]
    pub filename: String,

    /// The workout to add, i.e. "4rd 21 box jump over, 15 bar mu".
    #[arg(required = true)]
    pub workout: String,

    /// Comments for a workout, i.e. "T.C. 15'" or "Instead of Bar Mu do Other Movement".
    #[arg(short, long, default_value = None)]
    pub comments: Option<String>,

    /// Name for the workout, i.e. "Fran", or "Open 25.2" if any.
    #[arg(short, long, default_value = None)]
    pub name: Option<String>,
}

#[derive(Parser, Debug)]
pub struct ListCommand {
    /// Whether to list the workouts or generate a markdown page for them.
    #[arg(short, long, default_value = "true")]
    pub page: bool,
}
