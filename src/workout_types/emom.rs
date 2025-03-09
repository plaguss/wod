use std::fmt;
use std::str::FromStr;

use crate::workout_types::rest::Rest;
/// Represents an Every Minute On the Minute (EMOM) workout.
///
/// The `EMOM` struct contains the number of minutes to perform the workout,
/// the interval at which the exercise is performed, whether the workout is
/// alternating between exercises, and the rest period between exercises.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use wod::workout_types::emom::EMOM;
///
/// let emom1 = EMOM::from_str("emom-10").unwrap();
/// assert_eq!(emom1.rounds, 10);
/// assert_eq!(emom1.every, 1);
/// assert_eq!(emom1.alternating, false);
/// assert_eq!(emom1.rest.duration, 0);
/// assert_eq!(emom1.rest.unit, "");
///
/// let emom2 = EMOM::from_str("emom-10-2").unwrap();
/// assert_eq!(emom2.rounds, 10);
/// assert_eq!(emom2.every, 2);
/// assert_eq!(emom2.alternating, false);
/// assert_eq!(emom2.rest.duration, 0);
/// assert_eq!(emom2.rest.unit, "");
///
/// let emom3 = EMOM::from_str("emom-10-30s").unwrap();
/// assert_eq!(emom3.rounds, 10);
/// assert_eq!(emom3.every, 1);
/// assert_eq!(emom3.alternating, false);
/// assert_eq!(emom3.rest.duration, 30);
/// assert_eq!(emom3.rest.unit, "s");
///
/// let emom4 = EMOM::from_str("emom-10-2-alt").unwrap();
/// assert_eq!(emom4.rounds, 10);
/// assert_eq!(emom4.every, 2);
/// assert_eq!(emom4.alternating, true);
/// assert_eq!(emom4.rest.duration, 0);
/// assert_eq!(emom4.rest.unit, "");
///
/// let emom5 = EMOM::from_str("emom-10-30s-alt").unwrap();
/// assert_eq!(emom5.rounds, 10);
/// assert_eq!(emom5.every, 1);
/// assert_eq!(emom5.alternating, true);
/// assert_eq!(emom5.rest.duration, 30);
/// assert_eq!(emom5.rest.unit, "s");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct EMOM {
    /// The number of rounds to perform the workout.
    pub rounds: u16,
    /// The interval at which the exercise is performed. Defaults to 1 minute.
    pub every: u16,
    /// A boolean indicating whether the workout is alternating between exercises.
    pub alternating: bool,
    /// A `Rest` struct representing the rest period between exercises.
    pub rest: Rest,
}

impl FromStr for EMOM {
    /// Creates a new `EMOM` instance from a string representation.
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into number part and name part
        let parts: Vec<&str> = s.split("-").collect();
        let mut alternating = false;
        let mut every = 1;
        let mut rounds = 1;
        let mut rest = Rest {
            duration: 0,
            unit: "".to_string(),
        };

        let mut counter = 0;
        for part in parts.iter() {
            match part {
                &"emom" => {
                    continue;
                }
                &"alt" => {
                    alternating = true;
                }
                _ => {
                    if part.contains("m") | part.contains("s") {
                        rest = Rest::from_str(part).expect("Invalid Rest format");
                        continue;
                    }

                    if counter == 0 {
                        rounds = part
                            .parse::<u16>()
                            .map_err(|_| "Invalid number format".to_string())?;
                    } else if counter == 1 {
                        every = part
                            .parse::<u16>()
                            .map_err(|_| "Invalid number format".to_string())?;
                    }
                    counter += 1;
                }
            }
        }
        Ok(EMOM {
            rounds,
            every,
            alternating,
            rest,
        })
    }
}

impl fmt::Display for EMOM {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "EMOM {} minutes", self.rounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emom() {
        assert_eq!(
            EMOM::from_str("emom-10").unwrap(),
            EMOM {
                rounds: 10,
                every: 1,
                alternating: false,
                rest: Rest {
                    duration: 0,
                    unit: "".to_string()
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: 1,
                alternating: true,
                rest: Rest {
                    duration: 0,
                    unit: "".to_string()
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-2").unwrap(),
            EMOM {
                rounds: 10,
                every: 2,
                alternating: false,
                rest: Rest {
                    duration: 0,
                    unit: "".to_string()
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-2-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: 2,
                alternating: true,
                rest: Rest {
                    duration: 0,
                    unit: "".to_string()
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-30s").unwrap(),
            EMOM {
                rounds: 10,
                every: 1,
                alternating: false,
                rest: Rest {
                    duration: 30,
                    unit: "s".to_string()
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-30s-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: 1,
                alternating: true,
                rest: Rest {
                    duration: 30,
                    unit: "s".to_string()
                }
            }
        );
    }

    #[test]
    fn test_emom_invalid() {
        assert!(EMOM::from_str("other-10").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                EMOM {
                    rounds: 10,
                    every: 1,
                    alternating: false,
                    rest: Rest {
                        duration: 0,
                        unit: "".to_string()
                    }
                }
            ),
            "EMOM 10 minutes"
        );
    }
}
