use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;
use wod::run_add_wod_from_file;

// Base tests to ensure the program runs correctly

#[test]
fn test_weightlifting_workout() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for our test files
    let temp_dir = TempDir::new()?;

    // Create test WOD file
    let wod_file_path = temp_dir.path().join("test.wod");
    let mut wod_file = File::create(&wod_file_path)?;
    writeln!(wod_file, "wl 3x(2+1) clean, split jerk @85%")?;

    // Create output path
    let output_path = temp_dir.path().join("workouts.md");

    // Run the function to test
    run_add_wod_from_file(
        output_path.clone(),
        wod_file_path.clone(),
        "2025-03-21".to_string(),
        None,
    )?;

    // Verify the file was created
    assert!(output_path.exists());

    // Read the content to verify it contains the expected workouts
    let content = fs::read_to_string(&output_path)?;

    // Checks of the metadata of the markdown file
    assert!(content.contains(
        r#"---
title: "workouts"
date: 2025-03-21
draft: false
---
"#
    ));
    // Checks on the contents from the workout
    assert!(content.contains(
        r#"

**Weightlifting**

3x(2+1) Clean + Split Jerk @ 85%
"#
    ));

    Ok(())
}

#[test]
fn test_for_time_workout() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for our test files
    let temp_dir = TempDir::new()?;

    // Create test WOD file
    let wod_file_path = temp_dir.path().join("test.wod");
    let mut wod_file = File::create(&wod_file_path)?;
    writeln!(wod_file, "ft 21 pull up, 42 du, 21 thruster @43kg, 18 chest to bar, 36 du, 18 thruster @51kg, 15 bar mu, 30 du, 15 thruster @61kg")?;

    // Create output path
    let output_path = temp_dir.path().join("workouts.md");

    // Run the function to test
    run_add_wod_from_file(
        output_path.clone(),
        wod_file_path.clone(),
        "2025-03-21".to_string(),
        None,
    )?;

    // Verify the file was created
    assert!(output_path.exists());

    // Read the content to verify it contains the expected workouts
    let content = fs::read_to_string(&output_path)?;

    // Checks the expected format for the wod
    assert!(content.contains(
        r#"
**For Time**

- 21 Pull Up

- 42 Double Under

- 21 Thruster @ 43kg

- 18 Chest To Bar

- 36 Double Under

- 18 Thruster @ 51kg

- 15 Bar Muscle Up

- 30 Double Under

- 15 Thruster @ 61kg
"#
    ));

    Ok(())
}

#[test]
fn test_run_add_wod_from_file_with_languages() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for our test files
    let temp_dir = TempDir::new()?;

    // Create test WOD file
    let wod_file_path = temp_dir.path().join("test_langs.wod");
    let mut wod_file = File::create(&wod_file_path)?;
    writeln!(wod_file, "wl 3x(2+1) clean, split jerk @85%")?;

    // Create output path base
    let output_path_base = temp_dir.path().join("workouts_lang");

    // Run the function with languages
    run_add_wod_from_file(
        output_path_base.clone(),
        wod_file_path.clone(),
        "2025-03-21".to_string(),
        Some("en,es".to_string()),
    )?;

    // Verify both language files were created
    let en_path = output_path_base.with_extension("md");
    let es_path = temp_dir.path().join("workouts_lang.es.md");

    assert!(en_path.exists());
    assert!(es_path.exists());

    // Read the content to verify both files contain the expected workouts
    let en_content = fs::read_to_string(&en_path)?;
    let es_content = fs::read_to_string(&es_path)?;

    assert_eq!(en_content, es_content);

    Ok(())
}

#[test]
fn test_run_add_wod_from_file_with_comments_and_names() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for our test files
    let temp_dir = TempDir::new()?;

    // Create test WOD file with comments and names
    let wod_file_path = temp_dir.path().join("test_comments.wod");
    let mut wod_file = File::create(&wod_file_path)?;
    writeln!(
        wod_file,
        "wl 3x(2+1) clean, split jerk @85%|Focus on technique|Heavy Day"
    )?;
    // writeln!(wod_file, "wl 4x2 front squat @85%|Deep position|")?;
    // writeln!(wod_file, "wl 3x4 push press @75%||Shoulder Work")?;

    // Create output path
    let output_path = temp_dir.path().join("workouts_comments.md");

    // Run the function to test
    run_add_wod_from_file(
        output_path.clone(),
        wod_file_path.clone(),
        "2025-03-21".to_string(),
        None,
    )?;

    // Verify the file was created
    assert!(output_path.exists());

    // Read the content to verify it contains the expected workouts with comments and names
    let content = fs::read_to_string(&output_path)?;

    // Check the name and comments are present
    assert!(content.contains(
        r#"
*Heavy Day*

**Weightlifting**

3x(2+1) Clean + Split Jerk @ 85%

Comments: *Focus on technique*
"#
    ));

    Ok(())
}

#[test]
fn test_run_add_wod_from_file_invalid_format() -> Result<(), Box<dyn std::error::Error>> {
    // Tests the wod file will have no workout data.
    // Create a temporary directory for our test files
    let temp_dir = TempDir::new()?;
    
    // Create test WOD file with invalid format (too many pipe separators)
    let wod_file_path = temp_dir.path().join("test_invalid.wod");
    let mut wod_file = File::create(&wod_file_path)?;
    writeln!(wod_file, "wl 3x(2+1) clean, split jerk @85%|comment|name|extra")?;
    
    // Create output path
    let output_path = temp_dir.path().join("workouts_invalid.md");
    
    // Run the function - it should work but log an error about the invalid line
    run_add_wod_from_file(
        output_path.clone(),
        wod_file_path.clone(),
        "2025-03-21".to_string(),
        None,
    )?;
    let content = fs::read_to_string(&output_path)?;
    // Count the number of jump of line, should be 9 if no wod was added
    let lines = content.split("\n").collect::<Vec<&str>>();
    assert_eq!(lines.len(), 9);
    // The file should still be created but without the invalid entry
    assert!(output_path.exists());
    
    Ok(())
}

#[test]
fn test_nonexistent_wodfile() {
    // Create a temporary directory for our test
    let temp_dir = TempDir::new().unwrap();
    
    // Create output path
    let output_path = temp_dir.path().join("workouts_nonexistent.md");
    
    // Try to run the function with a nonexistent WOD file
    let result = run_add_wod_from_file(
        output_path.clone(),
        PathBuf::from("nonexistent_file.wod"),
        "2025-03-21".to_string(),
        None,
    );

    // Verify it returns an error
    assert!(result.is_err());
    let err = result.unwrap_err();
    let err_string = err.to_string();
    assert!(err_string.contains("No such file or directory"));

}


// TODO: Tests for the format of different WODS