use std::str::{Chars, FromStr};

use crate::workout_type::{ForTime, WorkoutType};

#[derive(Debug, PartialEq)]
pub enum Token {
    // ft, amrap, emom, wl (what else?)
    WorkoutType(WorkoutType),
    // 21, (any number)
    Reps(String),
    // cal, m (meters), s (seconds)
    RepType(String),
    // pull up, thruster
    Movement(String),
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
            if c.is_alphanumeric() {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let workout_type = match result.as_str() {
            "amrap" => WorkoutType::Amrap,
            "emom" => WorkoutType::Emom,
            "wl" => WorkoutType::Weightlifting,
            _ => {
                // if not amrap, emom, wl, then it's a ForTimeType
                WorkoutType::ForTime(ForTime::from_str(result.as_str()).unwrap())
            }
        };
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

    fn read_number(&mut self) -> String {
        let mut result = String::new();

        while let Some(c) = self.current_char {
            if c.is_numeric() || c == '-' {
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
            } else if c.is_numeric() {
                let number = self.read_number();
                for rep in number.split('-') {
                    tokens.push(Token::Reps(rep.to_string()));
                }
                continue;
            } else if c.is_alphabetic() {
                let movement = self.read_movement();
                if !movement.is_empty() {
                    tokens.push(Token::Movement(movement));
                }
                continue;
            } else if c == ',' {
                self.advance();
                continue;
            } else {
                // Skip any other characters
                self.advance();
            }
        }

        tokens
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
                Token::WorkoutType(WorkoutType::ForTime(ForTime::from_str("ft").unwrap())),
                Token::Reps("21".to_string()),
                Token::Reps("15".to_string()),
                Token::Reps("9".to_string()),
                Token::Movement("pull up".to_string()),
                Token::Movement("thruster".to_string()),
            ]
        );
    }

    #[test]
    fn test_rounds() {
        let input = "5rd 20 double under, 30 cal row";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::WorkoutType(WorkoutType::ForTime(ForTime {
                    rounds: 5,
                    name: "rd".to_string()
                })),
                Token::Reps("20".to_string()),
                Token::Movement("double under".to_string()),
                Token::Reps("30".to_string()),
                Token::Movement("row".to_string()),
            ]
        );
    }

    // #[test]
    // fn test_amrap() {
    //     let input = "amrap-12m 10 db snatch, 1 ring muscle up";
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
