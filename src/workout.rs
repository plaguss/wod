use std::str::FromStr;

use crate::lexer::{Lexer, Token};
use crate::movement::Movement;
use crate::rep_types::rep_type::RepType;
use crate::rm::RM;
use crate::weight::Weight;
use crate::{ForTime, WorkoutType, AMRAP, EMOM};

#[derive(Debug, PartialEq)]
pub struct Workout {
    pub workout_type: WorkoutType,
    pub movements: Vec<Movement>,
    pub rep_types: Vec<RepType>,
    pub weights: Vec<Weight>,
    // TODO: These aren't clear yet
    pub x: Option<Vec<Token>>,
    pub at: Option<Vec<Token>>,
    pub plus: Option<Vec<Token>>,
    pub rm: Option<Vec<RM>>,
    tokens: Vec<Token>,
}

impl Workout {
    // TODO: Create a new method that takes the tokens and parses them.

    pub fn default() -> Self {
        Workout {
            workout_type: WorkoutType::from_str("ft").unwrap(),
            movements: Vec::new(),
            rep_types: Vec::new(),
            weights: Vec::new(),
            x: None,
            at: None,
            plus: None,
            rm: None,
            tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self, tokens: Vec<Token>) {
        // TODO: Update this to get the own the tokens instead of cloning them
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
                Token::Weight(weight) => {
                    self.weights.push(weight.clone());
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
                Token::Plus => {
                    if self.plus.is_none() {
                        self.plus = Some(Vec::new());
                    }
                    self.plus.as_mut().unwrap().push(Token::Plus);
                }
                Token::RM(rm) => {
                    if self.rm.is_none() {
                        self.rm = Some(Vec::new());
                    }
                    self.rm.as_mut().unwrap().push(rm.clone());
                }
                _ => {}
            }
        }
        self.tokens = tokens;
    }

    // Method to write the workout to a string to be printed
    pub fn write(&self) -> String {
        // Should be dependent on the workout type???
        let mut workout = String::new();

        match &self.workout_type {
            WorkoutType::ForTime(_ft) => {
                workout.push_str(&self.get_header("ft"));
                workout.push_str(self.write_for_time().as_str());
            }
            WorkoutType::Weightlifting => {
                workout.push_str(&self.get_header("wl"));
                workout.push_str(self.write_weightlifting().as_str());
            }
            WorkoutType::EMOM(_emom) => {
                workout.push_str(&self.get_header("emom"));
                workout.push_str(self.write_emom().as_str());
            }
            // WorkoutType::Amrap(_amrap) => {
            //     workout.push_str(self.write_amrap().as_str());
            // }
            _ => {
                eprintln!("Workout type not implemented {:?}", self.workout_type);
            }
        }

        workout
    }

    /// Get the header for the workout
    /// The header will be the workout type in bold, and the rest of the text in normal font.
    /// An initial --- separator to allow differentiating the blocks in the WOD.
    fn get_header(&self, workout_type: &str) -> String {
        if workout_type == "emom" {
            let header = format!("{}", self.workout_type);
            let separator = "\n\n";
            let formatted_header = header
                .split(separator)
                .enumerate()
                .map(|(index, part)| {
                    if index == 0 {
                        format!("**{}**", part)
                    } else {
                        part.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(separator);
            return format!("\n\n---\n\n{}\n\n", formatted_header);
        } else {
            return format!("\n\n---\n\n**{}**\n\n", self.workout_type);
        }
    }

    fn write_for_time(&self) -> String {
        // TODO: We need some kind of identifier for workouts that have reps informed as 21-15-9
        // Check this behaviour within the tokens
        fn check_contiguous_reps(tokens: &Vec<Token>) -> bool {
            // Helper function to check whether there are two contiguous movements/rep types,
            // e.g. 21-15-9 pull up, thruster, to help writing in such a format.
            tokens.windows(2).any(|window| {
                matches!(window[0], Token::RepType(_)) && matches!(window[1], Token::RepType(_))
            })
        }

        let mut workout = String::new();

        if check_contiguous_reps(&self.tokens) {
            // Format the Reps
            let reps_formatted = self
                .rep_types
                .iter()
                .map(|r| r.to_string())
                .collect::<Vec<_>>()
                .join("-");
            workout.push_str(&format!("{}\n\n", reps_formatted));
            // Format the Movements
            // In this case, the weights can be placed after the movements, and to associate
            // them we need to check the tokens order.

            // Iterate over the vector of tokens until we find the first Movement, from that point
            // we can start adding the movements and the weights.
            // To determine when to place the \n\n, a movement can have an associated weight.
            // If we are in a movement, and the previous movement was one, we add the \n\n
            let mut was_movement = false;
            for token in self.tokens.iter() {
                match token {
                    Token::Movement(movement) => {
                        if was_movement {
                            workout.push_str("\n\n");
                        }
                        workout.push_str(&format!("- {}", movement));
                        was_movement = true;
                    }
                    Token::Weight(weight) => {
                        was_movement = false;
                        workout.push_str(&format!(" At {}\n\n", weight));
                    }
                    _ => {}
                }
            }
            // Add the last \n\n if the last token was a movement
            if was_movement {
                workout.push_str("\n\n");
            }
        } else {
            let mut first_rep = true;
            // Format the Movements
            for token in self.tokens.iter().skip(1) {
                match token {
                    Token::RepType(rep_type) => {
                        if first_rep {
                            workout.push_str(&format!("- {} ", rep_type));
                            first_rep = false;
                        } else {
                            workout.push_str(&format!("\n\n- {} ", rep_type));
                        }
                    }
                    // We need to determine whether to add \n\n
                    Token::Movement(movement) => {
                        workout.push_str(&format!("{}", movement));
                    }
                    Token::Weight(weight) => {
                        workout.push_str(&format!(" At {}", weight));
                    }
                    _ => {}
                }
            }
            workout.push_str("\n\n");
        }
        workout
    }

    // Weightlifting workout type will be formatted using directly the tokens:
    // **Weightlifting**
    // Expects Rep Types, with any x or + in between, then the movements, and the weight
    fn write_weightlifting(&self) -> String {
        let mut workout = String::new();
        fn prepare_reps(
            rep_types: &Vec<RepType>,
            x: &Option<Vec<Token>>,
            plus: &Option<Vec<Token>>,
        ) -> String {
            // The simpler case: 3x3
            if plus.is_none() {
                let mut formatted = rep_types
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("x");
                formatted.push(' ');
                return formatted;
            }
            // Case of 2+2 (not sure I would write this way, but anyway)
            if x.is_none() {
                let mut formatted = rep_types
                    .iter()
                    .map(|m| m.to_string())
                    .collect::<Vec<_>>()
                    .join("+");
                formatted.push(' ');
                return formatted;
            }
            // Case of 2x(2+2)
            let mut reps_str = String::new();
            for (i, rep) in rep_types.iter().enumerate() {
                reps_str.push_str(&format!("{}", rep));
                if i == 0 {
                    reps_str.push_str("x(");
                    continue;
                }
                if i != rep_types.len() - 1 {
                    reps_str.push_str("+");
                }
            }
            reps_str.push_str(") ");
            reps_str
        }
        workout.push_str(&prepare_reps(&self.rep_types, &self.x, &self.plus));
        // Format the Movements as a , separated list
        let movements = self
            .movements
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        workout.push_str(&format!("{}", movements));
        // NOTE: Could there be more than one weight?
        workout.push_str(&format!(" At {}\n\n", self.weights[0]));
        workout
    }

    fn write_emom(&self) -> String {
        let mut workout = String::new();
        // Format the Rounds
        println!("{:?}", self.workout_type);
        let mut first_rep = true;
        // TODO: This is the same function used in the For Time (general case),
        // Refactor to avoid code duplication
        for token in self.tokens.iter().skip(1) {
            match token {
                Token::RepType(rep_type) => {
                    if first_rep {
                        workout.push_str(&format!("- {} ", rep_type));
                        first_rep = false;
                    } else {
                        workout.push_str(&format!("\n\n- {} ", rep_type));
                    }
                }
                // We need to determine whether to add \n\n
                Token::Movement(movement) => {
                    workout.push_str(&format!("{}", movement));
                }
                Token::Weight(weight) => {
                    workout.push_str(&format!(" At {}", weight));
                }
                _ => {}
            }
        }
        workout.push_str("\n\n");

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

        let expected = "\n\n---\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_create_workout() {
        let workout = "ft 21-15-9 pull up, thruster @ 43/30kg";
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
            weights: vec![Weight::from_str("43/30kg").unwrap()],
            x: None,
            at: vec![Token::At].into(),
            plus: None,
            rm: None,
            tokens: vec![
                Token::WorkoutType(WorkoutType::from_str("ft").unwrap()),
                Token::RepType(RepType::from_str("21").unwrap()),
                Token::RepType(RepType::from_str("15").unwrap()),
                Token::RepType(RepType::from_str("9").unwrap()),
                Token::Movement(Movement::from_str("pull up").unwrap()),
                Token::Movement(Movement::from_str("thruster").unwrap()),
                Token::At,
                Token::Weight(Weight::from_str("43/30kg").unwrap()),
            ],
        };

        assert_eq!(create_workout(workout), expected);
    }
}
