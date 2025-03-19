use std::str::FromStr;

use crate::lexer::{Lexer, LexerError, Token};
use crate::movement::Movement;
use crate::rep_types::rep_type::RepType;
use crate::rm::RM;
use crate::weight::Weight;
use crate::WorkoutType;

/// Represents a structured workout with various components such as movements, repetitions,
/// and weights. Supports different workout types like "For Time", "EMOM", "Weightlifting", etc.
///
/// This struct stores both the parsed workout data and the original tokens, allowing
/// for both structured access to workout components and the ability to format the workout
/// in a human-readable way.
///
/// # Examples
///
/// ```
/// use wod::{Workout, WorkoutType, RepType, Movement, Weight};
/// use wod::lexer::Token;
///
/// let tokens = vec![
///     Token::WorkoutType("ft".parse::<WorkoutType>().unwrap()),
///     Token::RepType("21".parse::<RepType>().unwrap()),
///     Token::Movement("pull up".parse::<Movement>().unwrap()),
///     Token::RepType("21".parse::<RepType>().unwrap()),
///     Token::Movement("thruster".parse::<Movement>().unwrap()),
///     Token::Weight("95lb".parse::<Weight>().unwrap()),
/// ];
///
/// let mut workout = Workout::new(tokens, Some("Fran".to_string()));
/// workout.parse();
///
/// println!("{}", workout.write());
/// ```
#[derive(Debug, PartialEq)]
pub struct Workout {
    /// The type of workout (ForTime, EMOM, Weightlifting, etc.)
    pub workout_type: WorkoutType,
    /// List of movements included in the workout
    pub movements: Vec<Movement>,
    /// List of repetition types
    pub rep_types: Vec<RepType>,
    /// List of weights used in the workout
    pub weights: Vec<Weight>,
    /// Optional collection of "x" tokens that may appear in the workout description
    /// (e.g., "3x5" repetition scheme)
    pub x: Option<Vec<Token>>,
    /// Optional collection of "@" tokens that may appear in the workout description
    /// (e.g., "back squat @100kg")
    pub at: Option<Vec<Token>>,
    /// Optional collection of "+" tokens that may appear in the workout description
    /// (e.g., "2+2+2" repetition scheme)
    pub plus: Option<Vec<Token>>,
    /// Optional collection of "RM" (repetition maximum) tokens
    /// (e.g., "5RM" for 5 repetition maximum)
    pub rm: Option<Vec<RM>>,
    /// The raw tokens that make up the workout, preserving the original structure
    tokens: Vec<Token>,
    /// Optional comments/notes about the workout
    comments: Option<String>,
    /// Optional name of the workout. Some workouts are given a name, i.e. "Fran".
    name: Option<String>,
}

impl Default for Workout {
    /// Creates a default Workout with "For Time" workout type and empty collections.
    fn default() -> Self {
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
            comments: None,
            name: None,
        }
    }
}

impl Workout {
    /// Creates a new Workout from a vector of tokens and optional comments.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of `Token` that represents the workout components
    /// * `comments` - Optional comments or notes about the workout
    /// * `name` - Optional name for the workout
    ///
    /// # Returns
    ///
    /// A new `Workout` instance with the given tokens and comments
    pub fn new(tokens: Vec<Token>, comments: Option<String>, name: Option<String>) -> Self {
        let mut wkt = Workout::default();
        wkt.tokens = tokens;
        wkt.comments = comments;
        wkt.name = name;
        wkt
    }

    /// Parses the tokens stored in the workout and populates the structured fields.
    ///
    /// This method analyzes the tokens vector and extracts specific workout components
    /// like workout type, movements, repetition types, weights, etc. into their respective
    /// fields for easier access and manipulation.
    pub fn parse(&mut self) {
        for token in &self.tokens {
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
            }
        }
    }

    /// Formats the workout into a human-readable string representation.
    ///
    /// The formatting depends on the workout type (ForTime, EMOM, Weightlifting, etc.)
    /// and includes headers, movement descriptions, repetition schemes, and comments.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the workout.
    pub fn write(&self) -> String {
        // Start from a markdown section separator
        let mut workout = String::from("---");

        if self.name.is_some() {
            workout.push_str(&format!("\n\n*{}*", self.name.as_ref().unwrap()));
        }

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

        if self.comments.is_some() {
            workout.push_str(&self.get_comments());
        }

        workout
    }

    /// Generates a formatted header for the workout based on its type.
    ///
    /// # Arguments
    ///
    /// * `workout_type` - A string representing the workout type abbreviation
    ///
    /// # Returns
    ///
    /// A formatted header string with appropriate markdown formatting.
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
            format!("\n\n{}\n\n", formatted_header)
        } else {
            format!("\n\n**{}**\n\n", self.workout_type)
        }
    }

    /// Formats a "For Time" workout into a human-readable string.
    ///
    /// Handles special cases like "21-15-9" rep schemes and combines movements
    /// with their associated weights and repetitions.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the "For Time" workout.
    fn write_for_time(&self) -> String {
        // TODO: We need some kind of identifier for workouts that have reps informed as 21-15-9
        // Check this behaviour within the tokens
        fn check_contiguous_reps(tokens: &[Token]) -> bool {
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
                        workout.push_str(&format!(" @ {}\n\n", weight));
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
                        workout.push_str(&format!(" @ {}", weight));
                    }
                    _ => {}
                }
            }
            workout.push_str("\n\n");
        }
        workout
    }

    /// Formats a "Weightlifting" workout into a human-readable string.
    ///
    /// Handles repetition schemes like "3x3", "2+2", or "2x(2+2)" and combines
    /// movements with their associated weights.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the "Weightlifting" workout.
    fn write_weightlifting(&self) -> String {
        let mut workout = String::new();
        fn prepare_reps(
            rep_types: &[RepType],
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
                    reps_str.push('+');
                }
            }
            reps_str.push_str(") ");
            reps_str
        }
        workout.push_str(&prepare_reps(&self.rep_types, &self.x, &self.plus));
        // Format the Movements as a + separated list
        let movements = self
            .movements
            .iter()
            .map(|m| m.to_string())
            .collect::<Vec<_>>()
            .join(" + ");

        workout.push_str(&movements.to_string());
        // NOTE: Could there be more than one weight?
        workout.push_str(&format!(" @ {}\n\n", self.weights[0]));
        workout
    }

    /// Formats an "EMOM" (Every Minute On the Minute) workout into a human-readable string.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the "EMOM" workout.
    fn write_emom(&self) -> String {
        let mut workout = String::new();
        // Format the Rounds
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
                    workout.push_str(&format!(" @ {}", weight));
                }
                _ => {}
            }
        }
        workout.push_str("\n\n");

        workout
    }

    /// Formats the workout comments into a human-readable string.
    ///
    /// # Returns
    ///
    /// A formatted string representation of the workout comments.
    fn get_comments(&self) -> String {
        let prepared_contents = self.comments.as_ref().unwrap();
        let comments: String = if prepared_contents.contains("\n") {
            prepared_contents
                .split("\n")
                .map(|part| format!("*{}*", part))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            format!("*{}*", prepared_contents)
        };

        format!("Comments: {}\n\n", comments)
    }
}

/// Creates a `Workout` object from a workout string and optional comments.
///
/// This function parses a workout string into a structured `Workout` object by:
/// 1. Tokenizing the input string with a `Lexer`
/// 2. Creating a new `Workout` from the tokens and optional comments
/// 3. Parsing the tokens to populate the `Workout` structure
///
/// # Arguments
/// * `workout` - A string slice containing the workout description to be parsed
/// * `comments` - Optional comments to be associated with the workout
///
/// # Returns
/// * `Result<Workout, LexerError>` - A `Workout` object if parsing succeeds, or a `LexerError` if tokenization fails
///
/// # Examples
///
/// ```
/// use wod::{create_workout, WorkoutType};
///
/// let workout = "ft 21-15-9 pull up, thruster @ 43/30kg";
/// let result = create_workout(workout, None);
/// assert!(result.is_ok());
///
/// let workout_obj = result.unwrap();
/// assert_eq!(workout_obj.workout_type, "ft".parse::<WorkoutType>().unwrap());
/// ```
///
/// # Errors
///
/// This function will return a `LexerError` if the `Lexer` fails to tokenize the input string.
pub fn create_workout(
    workout: &str,
    comments: Option<String>,
    name: Option<String>,
) -> Result<Workout, LexerError> {
    let mut lexer = Lexer::new(workout);
    let tokens = lexer.tokenize()?;
    let mut workout = Workout::new(tokens, comments, name);
    workout.parse();
    Ok(workout)
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

        let mut workout = Workout::new(tokens, None, None);
        workout.parse();

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

        let mut workout = Workout::new(tokens, None, None);
        workout.parse();

        let expected = "---\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster\n\n";
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
            comments: None,
            name: None,
        };

        assert_eq!(create_workout(workout, None, None).unwrap(), expected);
    }

    #[test]
    fn test_create_workout_error() {
        let workout = "ft 21-15-9 pulup, thruster @ 43/30kg";
        let expected =
            "Invalid Movement: Invalid movement: `pulup`, did you mean: `pull up`?".to_string();
        let workout = create_workout(workout, None, None);
        assert_eq!(workout.unwrap_err().to_string(), expected);
    }

    #[test]
    fn test_create_workout_with_comments() {
        let workout_str = "ft 21-15-9 pull up, thruster @ 43/30kg";
        let expected =
        "---\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster At 43/30kg\n\nComments: *blabla*\n\n".to_string();
        let comments = Some("blabla".to_string());
        let workout = create_workout(workout_str, comments, None).unwrap();
        let content = workout.write();
        assert_eq!(content, expected);

        // Test the case of \n in the comments.
        let expected =
        "---\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster At 43/30kg\n\nComments: *blabla*\n*other line*\n\n".to_string();
        let comments = Some("blabla\nother line".to_string());
        let workout = create_workout(workout_str, comments, None).unwrap();
        let content = workout.write();
        assert_eq!(content, expected);
    }

    #[test]
    fn test_create_workout_with_name() {
        let workout_str = "ft 21-15-9 pull up, thruster @ 43/30kg";
        let expected =
            "---\n\n*Fran*\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster At 43/30kg\n\n"
                .to_string();
        let name = Some("Fran".to_string());
        let workout = create_workout(workout_str, None, name).unwrap();
        let content = workout.write();
        assert_eq!(content, expected);
    }
}
