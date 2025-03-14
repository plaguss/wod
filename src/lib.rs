mod tests;

pub mod lexer;
pub mod movement;
pub mod rep_types;
pub mod rm;
pub mod weight;
pub mod workout;
pub mod workout_types;

pub use self::movement::Movement;
pub use self::rm::RM;
pub use self::weight::Weight;
pub use self::workout::{create_workout, Workout};

pub use self::workout_types::{
    amrap::AMRAP, emom::EMOM, for_time::ForTime, rest::Rest, workout_type::WorkoutType,
};

pub use self::rep_types::{cals::Cals, distance::Distance, rep_type::RepType, reps::Reps};

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;

use chrono::Local;

fn today() -> String {
    Local::now().format("%d-%m-%Y").to_string()
}

/// Get the default filename for the workout of the day
/// The filename is in the format "wod-<date>.md"
pub fn default_filename() -> String {
    format!("wod-{}.md", today())
}

/// Run the default program, "wod '<date>-<filename>.md'", creates a markdown file with a basic template.
///
/// This function ensures that the file has a `.md` extension and creates it if it doesn't already exist.
/// If the file exists and `force` is `false`, it will print a message and do nothing.
/// If the directory for the file does not exist, it will be created.
///
/// # Arguments
///
/// * `filename` - A mutable `PathBuf` representing the path to the file.
/// * `force` - A reference to a boolean indicating whether to overwrite the file if it exists.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the file is successfully created or already exists.
///   Returns an error if there are any issues creating the directory or writing to the file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use wod::run_base;
///
/// // Example usage
/// let mut filename = PathBuf::from("workout.md");
/// let force = false;
///
/// // match run_base(filename, &force) {
/// //     Ok(_) => println!("File created successfully or already exists."),
/// //     Err(e) => eprintln!("Error: {}", e),
/// // }
/// ```
pub fn run_base(mut filename: PathBuf, force: &bool) -> Result<(), Box<dyn std::error::Error>> {
    if filename
        .extension()
        .map_or(String::from("default_extension"), |ext| {
            ext.to_string_lossy().into_owned()
        })
        != "md"
    {
        filename.set_extension("md");
    }

    if filename.exists() && !force {
        println!("File '{}' already exists", filename.display());
        // Don't recreate the file
        return Ok(());
    }

    let dir_path = filename.parent().unwrap();

    // Create parent dir if doesn't exist
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }
    // Create the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename)?;

    // Write the markdown header of the file
    file.write_all(
        format!(
            r#"---
title: "{}"
date: {}
draft: false
---

Workout for the day, {}.

"#,
            today(),
            today(),
            today()
        )
        .as_bytes(),
    )?;

    Ok(())
}

/// Appends a new workout to a file.
///
/// This function takes a `filename` and a `workout` string, creates a workout
/// using the `create_workout` function, writes the workout to the specified file,
/// and appends it if the file already exists. If the file does not exist, it will
/// be created.
///
/// # Arguments
///
/// * `filename` - A `PathBuf` representing the path to the file where the workout
///                will be appended.
/// * `workout` - A string slice representing the workout to be added.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the operation
///                                              is successful, or an error if
///                                              something goes wrong.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use wod::run_add_workout;
///
/// // let filename = PathBuf::from("workouts.txt");
/// // let workout = "wl 3x4 push press @75%";
/// // run_add_workout(filename.clone(), workout).expect("Failed to add workout");
pub fn run_add_workout(filename: PathBuf, workout: &str) -> Result<(), Box<dyn std::error::Error>> {
    let wkt = create_workout(workout);
    let content = wkt.write();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}

/// Creates a WOD from a file containing the workouts line by line.
///
/// This function takes a `filename` and a `wodfile`, creates a workout.
/// using the `create_workout` function, writes the workout to the specified file,
/// and appends it if the file already exists. If the file does not exist, it will
/// be created.
///
/// # Arguments
///
/// * `filename` - A `PathBuf` representing the path to the file where the workout
///                will be appended.
/// * `wodfile` - A `PathBuf` representing the path to the file containing the workouts.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns `Ok(())` if the operation
///                                              is successful, or an error if
///                                              something goes wrong.
///
/// # Examples
///
/// Suppose we have a file named `example_wod.wod` with the following content:
///
/// ```text
/// wl 3x(2+1) clean, split jerk @85%
/// wl 4x2 front squat @85%
/// wl 3x4 push press @75%
/// ft 21 pull up, 42 du, 21 thruster @43kg, 18 chest to bar, 36 du, 18 thruster @51kg, 15 bar mu, 30 du, 15 thruster @61kg
/// ```
///
/// ```
/// use std::path::PathBuf;
/// use wod::run_add_wod_from_file;
///
/// // let filename = PathBuf::from("workouts.md");
/// // let wodfile = PathBuf::from(".example_wod.wod");
/// // run_add_wod_from_file(filename.clone(), wodfile.clone()).expect("Failed create WOD from file");
pub fn run_add_wod_from_file(
    filename: PathBuf,
    wodfile: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = run_base(filename.clone(), &true);

    if let Ok(lines) = read_wodfile(wodfile) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let _ = run_add_workout(filename.clone(), &line);
        }
    }
    Ok(())
}

fn read_wodfile(filename: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
