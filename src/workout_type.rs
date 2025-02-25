use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum WorkoutType {
    ForTime(ForTime),
    Amrap,
    Emom,
    Weightlifting,
}

impl fmt::Display for WorkoutType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkoutType::ForTime(ft) => write!(formatter, "{}", ft),
            WorkoutType::Amrap => write!(formatter, "Amrap"),
            WorkoutType::Emom => write!(formatter, "Emom"),
            WorkoutType::Weightlifting => write!(formatter, "Weightlifting"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForTime {
    pub rounds: u32,
    pub name: String,
}

impl FromStr for ForTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Find the first non-digit character
        let first_non_digit = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());

        // Split the string into number part and name part
        let (number_part, name_part) = s.split_at(first_non_digit);

        // Parse the rounds
        let rounds = if number_part.is_empty() {
            1 // Default to 1 if no number is present
        } else {
            number_part
                .parse::<u32>()
                .map_err(|_| "Invalid number format".to_string())?
        };

        Ok(ForTime {
            rounds,
            name: name_part.to_string(),
        })
    }
}

impl fmt::Display for ForTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rounds > 1 {
            return write!(formatter, "{} rounds for time", self.rounds);
        }
        write!(formatter, "For Time")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_for_time() {
        assert_eq!(
            "ft".parse::<ForTime>().unwrap(),
            ForTime {
                rounds: 1,
                name: "ft".to_string()
            }
        );
        assert_eq!(
            "5rd".parse::<ForTime>().unwrap(),
            ForTime {
                rounds: 5,
                name: "rd".to_string()
            }
        );
        assert_eq!(
            "10rd".parse::<ForTime>().unwrap(),
            ForTime {
                rounds: 10,
                name: "rd".to_string()
            }
        );
    }

    #[test]
    fn test_for_time_display() {
        assert_eq!(
            format!(
                "{}",
                ForTime {
                    rounds: 1,
                    name: "ft".to_string()
                }
            ),
            "For Time"
        );
        assert_eq!(
            format!(
                "{}",
                ForTime {
                    rounds: 5,
                    name: "rd".to_string()
                }
            ),
            "5 rounds for time"
        );
        assert_eq!(
            format!(
                "{}",
                ForTime {
                    rounds: 10,
                    name: "rd".to_string()
                }
            ),
            "10 rounds for time"
        );
    }

    #[test]
    fn test_workout_type_display() {
        assert_eq!(
            format!(
                "{}",
                WorkoutType::ForTime(ForTime {
                    rounds: 1,
                    name: "ft".to_string()
                })
            ),
            "For Time"
        );
        assert_eq!(format!("{}", WorkoutType::Amrap), "Amrap");
        assert_eq!(format!("{}", WorkoutType::Emom), "Emom");
        assert_eq!(format!("{}", WorkoutType::Weightlifting), "Weightlifting");
    }
}
