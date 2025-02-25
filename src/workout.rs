use crate::lexer::Token;
use crate::movement::Movement;
use crate::workout_type::{ForTime, WorkoutType};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub movements: Vec<Movement>,
    // To be updated
    pub reps: Vec<String>,
    pub rep_types: Vec<String>,
}

impl Workout {
    pub fn default() -> Self {
        Workout {
            workout_type: WorkoutType::Emom,
            movements: Vec::new(),
            reps: Vec::new(),
            rep_types: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        // When is time to print the workout, will have to order the things here
        for token in &tokens {
            match token {
                Token::WorkoutType(workout_type) => {
                    self.workout_type = workout_type.clone();
                }
                Token::Movement(movement) => {
                    self.movements.push(Movement::from_str(movement).unwrap());
                }
                Token::Reps(rep) => {
                    self.reps.push(rep.to_string());
                }
                Token::RepType(rep_type) => {
                    self.rep_types.push(rep_type.to_string());
                }
            }
        }
    }

    // Method to write the workout to a string to be printed
    pub fn write(&self) -> String {
        // Should be dependent on the workout type???
        let mut workout = String::new();
        workout.push_str(&format!("**{}**\n", self.workout_type));

        // Continue with the types of workouts to print
        // TODO: Separate in different write_for_time... write_amrap... methods

        match &self.workout_type {
            WorkoutType::ForTime(ft) => {
                workout.push_str(self.write_for_time().as_str());
            }
            _ => {
                // Default print, should be dependent on the workout type
                for (i, movement) in self.movements.iter().enumerate() {
                    workout.push_str(&format!("{}: {}\n", i + 1, movement));
                }
                for (i, rep) in self.reps.iter().enumerate() {
                    workout.push_str(&format!("{}: {}\n", i + 1, rep));
                }
                for (i, rep_type) in self.rep_types.iter().enumerate() {
                    workout.push_str(&format!("{}: {}\n", i + 1, rep_type));
                }
            }
        }

        workout
    }

    fn write_for_time(&self) -> String {
        let mut workout = String::new();
        workout.push_str(&format!("{}\n\n", self.reps.join("-")));
        // Format the Movements
        // for (i, rep) in self.reps.iter().enumerate() {
        //     workout.push_str(&format!("{}: {}\n", i + 1, rep));
        // }
        for movement in self.movements.iter() {
            workout.push_str(&format!("- {}\n\n", movement));
        }
        workout
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workout_type::{ForTime, WorkoutType};

    #[test]
    fn test_workout_parse() {
        let tokens = vec![
            Token::WorkoutType(WorkoutType::ForTime(ForTime::from_str("ft").unwrap())),
            Token::Reps("21".to_string()),
            Token::Reps("15".to_string()),
            Token::Reps("9".to_string()),
            Token::Movement("pull up".to_string()),
            Token::Movement("thruster".to_string()),
        ];

        let mut workout = Workout::default();
        workout.parse(tokens);

        assert_eq!(workout.movements.len(), 2);
        assert_eq!(workout.reps.len(), 3);
        assert_eq!(workout.rep_types.len(), 0);
        assert_eq!(
            workout.workout_type,
            WorkoutType::ForTime(ForTime {
                rounds: 1,
                name: "ft".to_string()
            })
        );
    }

    #[test]
    fn test_workout_write() {
        let tokens = vec![
            Token::WorkoutType(WorkoutType::ForTime(ForTime::from_str("ft").unwrap())),
            Token::Reps("21".to_string()),
            Token::Reps("15".to_string()),
            Token::Reps("9".to_string()),
            Token::Movement("pull up".to_string()),
            Token::Movement("thruster".to_string()),
        ];

        let mut workout = Workout::default();
        workout.parse(tokens);

        let expected = r#"ForTime: ft\n1: PullUp\n2: Thruster\n1: 21\n2: 15\n3: 9\n"#;
        assert_eq!(workout.write(), expected);
    }
}
