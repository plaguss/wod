mod tests;

// pub mod distance;
pub mod lexer;
pub mod movement;
pub mod rep_types;
pub mod rm;
pub mod weight;
pub mod workout;
pub mod workout_types;

// TODO: Redirect methods for easier import
pub use self::workout_types::{amrap::AMRAP, workout_type::WorkoutType};

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use chrono::Local;

use crate::lexer::Lexer;
use crate::workout::Workout;

fn today() -> String {
    Local::now().format("%d-%m-%Y").to_string()
}

pub fn default_filename() -> String {
    format!("wod-{}.md", today())
    // PathBuf::from(format!("wod-{}.md", today()))
}

// Run the default program, "wod 'date-filename.md'"
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
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    // Write the markdown header of the file
    // TODO: Create a template string for the file content
    file.write_all(
        // Currently there's no way of informing the categories/tags, let it for later
        format!(
            r#"---
title: "WOD for {}"
date: {}
draft: false
---

"#,
            today(),
            today()
        )
        .as_bytes(),
    )?;

    Ok(())
}

pub fn run_add_workout(filename: PathBuf, workout: &str) -> Result<(), Box<dyn std::error::Error>> {
    // This command should add a workout to the file passed as argument
    // If the file doesn't exist, it should create it
    // If the file exists, it should append the workout to the file
    let mut lexer = Lexer::new(workout);
    let tokens = lexer.tokenize();
    let mut workout = Workout::default();
    workout.parse(tokens);
    let content = workout.write();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&filename)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
