use std::str::FromStr;

use crate::lexer::{Lexer, Token};
use crate::movement::Movement;
use crate::rep_types::rep_type::RepType;
use crate::weight::Weight;
use crate::workout_types::workout_type::WorkoutType;

#[derive(Debug, PartialEq)]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub movements: Vec<Movement>,
    pub rep_types: Vec<RepType>,
    pub weights: Vec<Weight>,
    // TODO: These aren't clear yet
    pub x: Option<Vec<Token>>,
    pub at: Option<Vec<Token>>,
    pub rm: Option<Vec<Token>>,
}

impl Workout {
    pub fn default() -> Self {
        Workout {
            workout_type: WorkoutType::from_str("ft").unwrap(),
            movements: Vec::new(),
            rep_types: Vec::new(),
            weights: Vec::new(),
            x: None,
            at: None,
            rm: None,
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
                    self.movements.push(movement.clone());
                }
                Token::RepType(rep_type) => {
                    self.rep_types.push(rep_type.clone());
                }
                Token::X => {
                    if self.x.is_none() {
                        self.x = Some(Vec::new());
                    }
                    self.x.as_mut().unwrap().push(Token::X);
                }
                Token::At => {
                    if self.at.is_none() {
                        self.at = Some(Vec::new());
                    }
                    self.at.as_mut().unwrap().push(Token::At);
                }
                _ => {}
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
            WorkoutType::ForTime(_ft) => {
                workout.push_str(self.write_for_time().as_str());
            }
            WorkoutType::Weightlifting => {
                workout.push_str(self.write_weightlifting().as_str());
            }
            // WorkoutType::Amrap(_amrap) => {
            //     workout.push_str(self.write_amrap().as_str());
            // }
            // WorkoutType::Emom => {
            //     workout.push_str(self.write_emom().as_str());
            // }
            _ => {
                // Default print, should be dependent on the workout type
                for (i, movement) in self.movements.iter().enumerate() {
                    workout.push_str(&format!("{}: {}\n", i + 1, movement));
                }
                for (i, rep) in self.rep_types.iter().enumerate() {
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

        let reps_formatted = self
            .rep_types
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join("-");
        workout.push_str(&format!("{}\n\n", reps_formatted));
        // Format the Movements
        for movement in self.movements.iter() {
            workout.push_str(&format!("- {}\n\n", movement));
        }
        workout
    }

    fn write_weightlifting(&self) -> String {
        let mut workout = String::new();
        // THIS WORKOUT TYPE ISN'T PROPERLY WRITTEN, DON'T KNOW YET HOW TO DO IT
        let reps_formatted = self
            .rep_types
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>()
            .join("--");
        workout.push_str(&format!("{}\n\n", reps_formatted));
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

/// Create a workout from a string
/// ADD EXAMPLE
pub fn create_workout(workout: &str) -> Workout {
    let mut lexer = Lexer::new(workout);
    let tokens = lexer.tokenize();
    let mut workout = Workout::default();
    workout.parse(tokens);
    workout
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rep_types::rep_type::RepType;
    use crate::workout_types::for_time::ForTime;
    use crate::workout_types::workout_type::WorkoutType;

    #[test]
    fn test_workout_parse() {
        let tokens = vec![
            Token::WorkoutType(WorkoutType::from_str("ft").unwrap()),
            Token::RepType(RepType::from_str("21").unwrap()),
            Token::RepType(RepType::from_str("15").unwrap()),
            Token::RepType(RepType::from_str("9").unwrap()),
            Token::Movement(Movement::from_str("pull up").unwrap()),
            Token::Movement(Movement::from_str("thruster").unwrap()),
        ];

        let mut workout = Workout::default();
        workout.parse(tokens);

        assert_eq!(workout.movements.len(), 2);
        assert_eq!(workout.rep_types.len(), 3);
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
            Token::WorkoutType(WorkoutType::from_str("ft").unwrap()),
            Token::RepType(RepType::from_str("21").unwrap()),
            Token::RepType(RepType::from_str("15").unwrap()),
            Token::RepType(RepType::from_str("9").unwrap()),
            Token::Movement(Movement::from_str("pull up").unwrap()),
            Token::Movement(Movement::from_str("thruster").unwrap()),
        ];

        let mut workout = Workout::default();
        workout.parse(tokens);

        let expected = "**For Time**\n21-15-9\n\n- Pull Up\n\n- Thruster\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_create_workout() {
        let workout = "ft 21-15-9 pull up, thruster";
        let expected = Workout {
            workout_type: WorkoutType::from_str("ft").unwrap(),
            movements: vec![
                Movement::from_str("pull up").unwrap(),
                Movement::from_str("thruster").unwrap(),
            ],
            rep_types: vec![
                RepType::from_str("21").unwrap(),
                RepType::from_str("15").unwrap(),
                RepType::from_str("9").unwrap(),
            ],
            weights: vec![],
            x: None,
            at: None,
            rm: None,
        };

        assert_eq!(create_workout(workout), expected);
    }
}
