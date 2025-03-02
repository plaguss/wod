use std::str::{Chars, FromStr};

// use crate::distance::Distance;
use crate::movement::Movement;
use crate::rep_types::rep_type::RepType;
use crate::rm::RM;
use crate::weight::Weight;
use crate::workout_types::workout_type::WorkoutType;

#[derive(Debug, PartialEq)]
pub enum Token {
    // ft, amrap, emom, wl (what else?)
    // TODO: The workout tye should deal with all this to avoid moving the logic to the lexer
    WorkoutType(WorkoutType),
    // 21, (any number)
    // cal, m (meters), s (seconds)
    RepType(RepType),
    // Distance(Distance), // 5k, 10m, 1mile
    // pull up, thruster
    Movement(Movement),
    // Special of weightlifting
    // 5x5, 3x(1+1), 1rm
    X,              // X in 5x$
    At,             // @ in @70%
    Plus,           // + in 3x(1+1)
    RM(RM),         // 1rm, these are dealt with in their own struct
    Weight(Weight), // 60kg, 60/40kg, 70%
}

pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        Lexer {
            input: chars,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn read_workout_type(&mut self) -> WorkoutType {
        let mut result = String::new();

        while let Some(c) = self.current_char {
            // To include the hyphen in the workout type
            if c.is_alphanumeric() || c == '-' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let workout_type = WorkoutType::from_str(result.as_str()).expect("Invalid workout type");
        workout_type
    }

    fn read_movement(&mut self) -> String {
        let mut result = String::new();

        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == ' ' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result.trim().to_string()
    }

    fn read_number_scheme(&mut self) -> String {
        // Read number will read until it finds a non-numeric character,
        // it takes into account the following cases:
        // 21-15-9
        // 5x5
        // 3x(1+1)
        // 1rm
        // 60kg
        // 60/40kg
        // 70%
        let mut result = String::new();

        while let Some(c) = self.current_char {
            if c.is_numeric()
                || matches!(
                    c.to_lowercase().next().unwrap(),
                    '-' | '+'
                        | '('
                        | ')'
                        | 'x'
                        | '/'
                        | 'k'
                        | 'g'
                        | '%'
                        | 'r'
                        | 'm'
                        | 'i'
                        | 'l'
                        | 'e'
                        | 'c'
                        | 'a'
                )
            {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }

        result
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut first_token = true;

        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if first_token {
                let workout_type = self.read_workout_type();
                tokens.push(Token::WorkoutType(workout_type));
                first_token = false;
                continue;
            }

            match c {
                '@' => {
                    // @70% or @60kg
                    tokens.push(Token::At);
                    self.advance();
                    continue;
                }
                c if c.is_numeric() => {
                    self.parse_numeric(&mut tokens);
                }
                c if c.is_alphabetic() => {
                    self.parse_alphabetic(&mut tokens);
                }
                _ => {
                    // Skip any other characters, like commas
                    self.advance();
                }
            }
        }

        tokens
    }

    fn parse_numeric(&mut self, tokens: &mut Vec<Token>) {
        fn process_buf(buf: &mut Vec<char>, tokens: &mut Vec<Token>) {
            if !buf.is_empty() {
                tokens.push(Token::RepType(
                    RepType::from_str(buf.iter().collect::<String>().as_str()).unwrap(),
                ));
                buf.clear();
            }
        }
        let number = self.read_number_scheme();
        // Workouts like 5x5, or 21-15-9 are parsed here
        if number.contains('x') {
            // In case of numbers, store the chars to cast them as a single number at the end
            let mut buf = Vec::new();
            for c in number.chars() {
                match c {
                    'x' => {
                        process_buf(&mut buf, tokens);
                        tokens.push(Token::X)
                    }
                    '+' => {
                        process_buf(&mut buf, tokens);
                        tokens.push(Token::Plus)
                    }
                    '(' | ')' => {
                        // Skip these tokens
                        process_buf(&mut buf, tokens);
                    }
                    _ => {
                        println!("c on buf: {}", c);
                        buf.push(c)
                    }
                }
            }
            // Push any pending number in the buffer
            process_buf(&mut buf, tokens);
        } else if number.contains("kg") || number.contains("%") {
            tokens.push(Token::Weight(Weight::from(number.to_string())));
        } else if number.contains("rm") {
            tokens.push(Token::RM(RM::from(number.to_string())));
        } else if number.contains("K")
            || number.contains("k")
            || number.contains("m")
            || number.contains("i")
            || number.contains("l")
            || number.contains("e")
        {
            tokens.push(Token::RepType(
                RepType::from_str(&number).expect("Invalid rep type"),
            ));
        } else {
            // Workouts like 21-15-9
            for rep in number.split('-') {
                tokens.push(Token::RepType(
                    RepType::from_str(rep).expect("Invalid rep type"),
                ));
            }
        }
    }

    fn parse_alphabetic(&mut self, tokens: &mut Vec<Token>) {
        let movement = self.read_movement();
        println!("movement: {}", movement);

        if !movement.is_empty() {
            if movement.contains("cal") {
                // Something like "30 cal row" will be split here
                let parts: Vec<_> = movement.split(' ').collect();
                tokens.push(Token::RepType(RepType::from_str(parts[0]).unwrap()));
                let mov = Movement::from_str(parts[1]).expect("Invalid movement");
                tokens.push(Token::Movement(mov));
            } else {
                let mov = Movement::from_str(&movement).expect("Invalid movement");
                tokens.push(Token::Movement(mov));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_time() {
        let input = "ft 21-15-9 pull up, thruster";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::from_str("ft").unwrap()),
                Token::RepType(RepType::from_str("21").unwrap()),
                Token::RepType(RepType::from_str("15").unwrap()),
                Token::RepType(RepType::from_str("9").unwrap()),
                Token::Movement(Movement::from_str("pull up").unwrap()),
                Token::Movement(Movement::from_str("thruster").unwrap()),
            ]
        );
    }

    #[test]
    fn test_rounds() {
        let input = "5rd 20 double under, 30cal row";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::from_str("5rd").unwrap()),
                Token::RepType(RepType::from_str("20").unwrap()),
                Token::Movement(Movement::from_str("double under").unwrap()),
                Token::RepType(RepType::from_str("30cal").unwrap()),
                Token::Movement(Movement::from_str("row").unwrap()),
            ]
        );
    }

    #[test]
    fn test_weightlifting_0() {
        let input = "wl 5x5 snatch @ 70%";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::Weightlifting),
                Token::RepType(RepType::from_str("5").unwrap()),
                Token::X,
                Token::RepType(RepType::from_str("5").unwrap()),
                Token::Movement(Movement::from_str("snatch").unwrap()),
                Token::At,
                Token::Weight(Weight::from("70%".to_string())),
            ]
        );
    }

    #[test]
    fn test_weightlifting_0_bigger_set() {
        let input = "wl 5x10 snatch @ 70%";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::Weightlifting),
                Token::RepType(RepType::from_str("5").unwrap()),
                Token::X,
                Token::RepType(RepType::from_str("10").unwrap()),
                Token::Movement(Movement::from_str("snatch").unwrap()),
                Token::At,
                Token::Weight(Weight::from("70%".to_string())),
            ]
        );
    }

    #[test]
    fn test_weightlifting_1() {
        let input = "wl 1rm snatch";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::Weightlifting),
                Token::RM(RM::from("1rm".to_string())),
                Token::Movement(Movement::from_str("snatch").unwrap()),
            ]
        );
    }

    #[test]
    fn test_weightlifting_2() {
        let input = "wl 3x(1+1+1) clean,front squat,split jerk @ 80kg";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::Weightlifting),
                Token::RepType(RepType::from_str("3").unwrap()),
                Token::X,
                Token::RepType(RepType::from_str("1").unwrap()),
                Token::Plus,
                Token::RepType(RepType::from_str("1").unwrap()),
                Token::Plus,
                Token::RepType(RepType::from_str("1").unwrap()),
                Token::Movement(Movement::from_str("clean").unwrap()),
                Token::Movement(Movement::from_str("front squat").unwrap()),
                Token::Movement(Movement::from_str("split jerk").unwrap()),
                Token::At,
                Token::Weight(Weight::from("80kg".to_string())),
            ]
        );
    }

    #[test]
    fn test_simple_running() {
        let input = "ft 5k run";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::from_str("ft").unwrap()),
                Token::RepType(RepType::from_str("5k").unwrap()),
                Token::Movement(Movement::from_str("run").unwrap()),
            ]
        )
    }
    #[test]
    fn test_amrap() {
        let input = "amrap-12 10 db snatch, 1 ring muscle up";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::from_str("amrap-12").unwrap()),
                Token::RepType(RepType::from_str("10").unwrap()),
                Token::Movement(Movement::from_str("db snatch").unwrap()),
                Token::RepType(RepType::from_str("1").unwrap()),
                Token::Movement(Movement::from_str("ring muscle up").unwrap()),
            ]
        );
    }

    // #[test]
    // fn test_emom() {
    //     let input = "emom-90s(-alt) 21-15-9 thrusters,pull ups";
    //     let mut lexer = Lexer::new(input);
    //     let tokens = lexer.tokenize();

    //     assert_eq!(tokens, vec![
    //         Token::WorkoutType("ft".to_string()),
    //         Token::Reps("21".to_string()),
    //         Token::Reps("15".to_string()),
    //         Token::Reps("9".to_string()),
    //         Token::Movement("pull up".to_string()),
    //         Token::Movement("thruster".to_string()),
    //     ]);
    // }

    // #[test]
    // fn test_strength() {
    //     let input = "wl 5x5 snatch @70%";
    //     let mut lexer = Lexer::new(input);
    //     let tokens = lexer.tokenize();

    //     assert_eq!(tokens, vec![
    //         Token::WorkoutType("ft".to_string()),
    //         Token::Reps("21".to_string()),
    //         Token::Reps("15".to_string()),
    //         Token::Reps("9".to_string()),
    //         Token::Movement("pull up".to_string()),
    //         Token::Movement("thruster".to_string()),
    //     ]);
    // }

    // #[test]
    // fn test_strength_2() {
    //     let input = "wl 2x(1+1) clean and jerk @70kg";
    //     let mut lexer = Lexer::new(input);
    //     let tokens = lexer.tokenize();

    //     assert_eq!(tokens, vec![
    //         Token::WorkoutType("ft".to_string()),
    //         Token::Reps("21".to_string()),
    //         Token::Reps("15".to_string()),
    //         Token::Reps("9".to_string()),
    //         Token::Movement("pull up".to_string()),
    //         Token::Movement("thruster".to_string()),
    //     ]);
    // }
}
