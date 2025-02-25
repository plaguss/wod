pub mod cli;
pub mod lexer;
pub mod movement;
pub mod tests;
pub mod utils;
pub mod workout;
pub mod workout_type;

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use crate::lexer::Lexer;
use crate::workout::Workout;
use crate::workout_type::{ForTime, WorkoutType};


pub fn run_add_workout(filename: &str, workout: &str) -> Result<(), Box<dyn std::error::Error>> {
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
