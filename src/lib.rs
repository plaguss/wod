mod tests;

pub mod lexer;
pub mod movement;
pub mod rep_types;
pub mod rm;
pub mod weight;
pub mod workout;
pub mod workout_types;

pub use self::movement::{Movement, MovementParseError};
pub use self::rm::RM;
pub use self::weight::Weight;
pub use self::workout::{create_workout, Workout};

pub use self::workout_types::{
    amrap::AMRAP, emom::EMOM, every::Every, for_time::ForTime, workout_type::WorkoutType,
};

pub use self::rep_types::{cals::Cals, distance::Distance, rep_type::RepType, reps::Reps};

use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::PathBuf;

use chrono::Local;

/// Returns today's date as "YYYY-MM-DD"
pub fn today() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

/// Get the default filename for the workout of the day
/// The filename is in the format "wod-<date>", without
/// the extension that will be added later
pub fn default_filename() -> String {
    // format!("wod-{}.md", today())
    format!("wod-{}", today())
}

fn get_languages(languages: &str) -> Vec<String> {
    languages.split(',').map(|s| s.to_string()).collect()
}

/// Run the default program, "wod '<date>-<filename>'", creates a markdown file with a basic template,
/// or multiple equal files if "languages" is provided.
///
/// This function ensures that the file has a `.md` extension and creates it if it doesn't already exist.
/// If `languages` is provided, it will generate a default file with `.md` extension,
/// plus one for each one of the languages.
/// If the file exists and `force` is `false`, it will print a message and do nothing.
/// If the directory for the file does not exist, it will be created.
///
/// # Arguments
///
/// * `filename` - A mutable `PathBuf` representing the path to the file.
/// * `force` - A reference to a boolean indicating whether to overwrite the file if it exists.
/// * `date` - Optional date to include in the file metadata. This file will be
///     used by Hugo to sort the pages. If not given, the current day will be used.
/// * `languages` - A list of ISO languages. This field is optional, if given, will be
///     used to generate duplicates of the file with the language extension so they
///     can be rendered in the Hugo blog. i.e. "es,it" for italian and spanish. English
///     will be generated by default, without extension. If "en,es" for example is passed,
///     a file without extension will represent "en", and another with file extension
///     "es.md" will correspond to the spanish one.
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
/// use wod::{run_base, today};
///
/// // Example usage
/// let mut filename = PathBuf::from("workout.md");
/// let force = false;
/// let date = today();
/// let languages: Option<String> = None; // Or Some("es".to_string) for english and spanish files
///
/// // match run_base(filename, &force, date, languages) {
/// //     Ok(_) => println!("File created successfully or already exists."),
/// //     Err(e) => eprintln!("Error: {}", e),
/// // }
/// ```
pub fn run_base(
    filename: PathBuf,
    force: &bool,
    date: String,
    languages: Option<String>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut filenames: Vec<PathBuf> = Vec::new();
    let langs = languages.map_or_else(
        || vec!["en".to_string()],
        |lang| get_languages(lang.as_str()),
    );

    // Creates a markdown file with the Hugo expected metadata.
    fn create_file(
        filename: &PathBuf,
        force: &bool,
        date: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        let title = filename
            .file_name()
            .unwrap()
            .to_string_lossy()
            .replace("wod-", "")
            .replace(".md", "");

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
                title, date, title
            )
            .as_bytes(),
        )?;
        Ok(())
    }

    for lang in langs {
        // Check if the filename has an extension:
        // If the language is english, just set .md as the file extension,
        // otherwise, the language will be part of the extension for the filename
        // so for spanish it will write "<filename>.es.md"
        let lang_filename = {
            let mut filename = filename.clone();
            if filename.extension().is_none() {
                let ext = if lang == "en" {
                    "md"
                } else {
                    &format!("{}.md", lang)
                };
                filename.set_extension(ext);
            }
            filename
        };

        create_file(&lang_filename, force, date.clone())?;
        filenames.push(lang_filename);
    }

    Ok(filenames)
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
/// // let comments = None;
/// // let name = None;
/// // run_add_workout(filename.clone(), workout).expect("Failed to add workout");
pub fn run_add_workout(
    filename: PathBuf,
    workout: &str,
    comments: Option<String>,
    name: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let wkt = create_workout(workout, comments, name);
    let content: String = match wkt {
        Ok(wkt) => wkt.write(),
        Err(e) => {
            eprintln!("While reading workout: '{}'", workout);
            eprintln!("Error: {:#?}", e);
            std::process::exit(1);
        }
    };

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
/// * `date` - A date that will be used as in Hugo's metadata to sort the files.
///     It must be informed in "YYYY-MM-DD", the CLI will fill this value with the current
///     day by default.
/// * `languages` - A comma separated list of languages, optional. If not informed
///     a single file will be generated without a language extension, corresponding to english,
///     otherwise, there will be created as much filenames as languages. i.e. "en,es" will
///     generate 2 copies of the filenames, one with ".md" and other with ".es.md" file
///     extension.
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
/// // run_add_wod_from_file(filename.clone(), wodfile.clone(), "2025-03-19".to_string()).expect("Failed create WOD from file");
pub fn run_add_wod_from_file(
    filename: PathBuf,
    wodfile: PathBuf,
    date: String,
    languages: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // If languages was used, more than one filename will be generated, and
    // we have to keep track of those when adding the workouts
    let filenames = run_base(filename.clone(), &true, date, languages)?;
    let lines = read_wodfile(wodfile)?;

    fn parse_line(line: &str) -> Result<(&str, Option<String>, Option<String>), WodFileError> {
        let sections: Vec<&str> = line.split('|').collect();
        let (workout, comments, name) = match sections.len() {
            1 => (sections[0], None, None),
            2 => (
                sections[0],
                if sections[1].is_empty() {
                    None
                } else {
                    Some(sections[1].to_string())
                },
                None,
            ),
            3 => (
                sections[0],
                if sections[1].is_empty() {
                    None
                } else {
                    Some(sections[1].to_string())
                },
                if sections[2].is_empty() {
                    None
                } else {
                    Some(sections[2].to_string())
                },
            ),
            _ => {
                return Err(WodFileError::InvalidFile(format!(
                    "Invalid format, expected 1-3 parts, got {}, content: '{}'",
                    sections.len(),
                    line
                )))
            }
        };

        Ok((workout, comments, name))
    }

    for line in lines.map_while(Result::ok) {
        match parse_line(&line) {
            Ok((workout, comments, name)) => {
                // To avoid rereading the file, wite the workout to each of the filenames
                for fname in filenames.iter() {
                    run_add_workout(fname.clone(), workout, comments.clone(), name.clone())?;
                }
            }
            Err(err) => {
                eprintln!("Error parsing line. {}", err);
            }
        }
    }
    println!("Created filenames from WOD file: {}", filename.display());
    Ok(())
}

fn read_wodfile(filename: PathBuf) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
enum WodFileError {
    InvalidFile(String),
}

impl Error for WodFileError {}

impl fmt::Display for WodFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WodFileError::InvalidFile(s) => write!(f, "Invalid wodfile: {}", s),
        }
    }
}

/// Generates a list of movements with explanatory videos in markdown format.
///
/// This function takes a `page` boolean to determine whether to create a markdown
/// page or just list the links. To generate a file just redirect the content.
///
/// # Arguments
///
/// * `page` - A `bool` to decide whether this is a markdown page prepared for a Hugo blog
///     or a list of markdown links.
///
/// # Returns
///
/// * `String` - The content as a string.
///
/// # Examples
///
/// Print the content to the console:
///
/// ```
/// use wod::run_create_list_movements;
///
/// let movement_list = run_create_list_movements(false);
/// let air_squat = movement_list.split("\n\n").next().unwrap();
/// assert_eq!(
///     air_squat,
///     "- [Air Squat](https://www.crossfit.com/essentials/the-air-squat)".to_string()
/// );
pub fn run_create_list_movements(page: bool) -> String {
    let mut content: String = "".to_string();
    if page {
        content.push_str(
            r#"---
title: "CrossFit Movements"
description: "List of movements with explanatory video"
---

List of CrossFit movements, click on them to see an explanation.

---

"#,
        )
    }
    content.push_str(
        Movement::list_with_url()
            .iter()
            .filter_map(|(key, value)| {
                if value.is_empty() {
                    None
                } else {
                    Some(format!("- [{}]({})", key, value))
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
            .as_str(),
    );
    content
}

#[cfg(test)]
mod test_cmd {
    use super::*;

    #[test]
    fn test_run_list_movements() {
        let result = run_create_list_movements(false);
        let air_squat = result.split("\n\n").next().unwrap();
        assert_eq!(
            air_squat,
            "- [Air Squat](https://www.crossfit.com/essentials/the-air-squat)".to_string()
        );
    }

    #[test]
    fn test_get_languages() {
        let langs = get_languages("en");
        assert_eq!(langs, vec!["en"]);

        let langs = get_languages("en,es");
        assert_eq!(langs, vec!["en", "es"]);
    }
}
