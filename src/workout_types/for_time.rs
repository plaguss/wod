use std::fmt;
use std::str::FromStr;

/// Represents a time-based exercise or workout configuration.
///
/// The "ForTime" struct is used to denote exercises or workouts that are performed
/// for a certain number of rounds within a time limit, or for time, where the goal
/// is to complete as many rounds as possible within a given time frame.
///
/// # Examples
///
/// ```
/// use wod::ForTime;
///
/// let ft = ForTime {
///     rounds: 1,
///     name: "ft".to_string(),
/// };
///
/// let rd = ForTime {
///     rounds: 5,
///     name: "rd".to_string(),
/// };
///
/// assert_eq!(ft.rounds, 1);
/// assert_eq!(ft.name, "ft");
/// assert_eq!(rd.rounds, 5);
/// assert_eq!(rd.name, "rd");
/// ```
///
/// ## Parsing
///
/// The "ForTime" struct implements the "FromStr" trait, the string should be formatted
/// as "<rounds><name>", where "<rounds>"
/// is an optional number of rounds, and "<name>" is the identifier for the exercise
/// or workout.
///
/// ```
/// use std::str::FromStr;
/// use wod::ForTime;
///
/// let ft: ForTime = "ft".parse().unwrap();
/// assert_eq!(ft, ForTime { rounds: 1, name: "ft".to_string() });
///
/// let rd: ForTime = "5rd".parse().unwrap();
/// assert_eq!(rd, ForTime { rounds: 5, name: "rd".to_string() });
/// ```
///
/// ## Display
///
/// The "ForTime" struct also implements the `Display` trait.
///
/// ```
/// use wod::ForTime;
///
/// let ft = ForTime {
///     rounds: 1,
///     name: "ft".to_string(),
/// };
///
/// let rd = ForTime {
///     rounds: 5,
///     name: "rd".to_string(),
/// };
///
/// assert_eq!(ft.to_string(), "For Time");
/// assert_eq!(rd.to_string(), "5 rounds for time");
/// ```
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
}
