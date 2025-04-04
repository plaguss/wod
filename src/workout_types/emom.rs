use std::fmt;
use std::str::FromStr;

use crate::workout_types::every::Every;

/// Represents an Every Minute On the Minute (EMOM) workout.
///
/// The `EMOM` struct contains the number of minutes to perform the workout,
/// the interval at which the exercise is performed, whether the workout is
/// alternating between exercises, and the rest period between exercises.
///
/// # Examples
///
/// ```
/// use wod::EMOM;
///
/// let emom1: EMOM = "emom-10".parse().unwrap();
/// assert_eq!(emom1.rounds, 10);
/// assert_eq!(emom1.every.duration, 1);
/// assert_eq!(emom1.alternating, false);
/// assert_eq!(emom1.rest.duration, 0);
/// assert_eq!(emom1.rest.unit, "");
///
/// let emom2: EMOM = "emom-10-2m".parse().unwrap();
/// assert_eq!(emom2.rounds, 10);
/// assert_eq!(emom2.every.duration, 2);
/// assert_eq!(emom2.alternating, false);
/// assert_eq!(emom2.rest.duration, 0);
/// assert_eq!(emom2.rest.unit, "");
///
/// let emom3: EMOM = "emom-10-r30s".parse().unwrap();
/// assert_eq!(emom3.rounds, 10);
/// assert_eq!(emom3.every.duration, 1);
/// assert_eq!(emom3.alternating, false);
/// assert_eq!(emom3.rest.duration, 30);
/// assert_eq!(emom3.rest.unit, "s");
///
/// let emom4: EMOM = "emom-10-2m-alt".parse().unwrap();
/// assert_eq!(emom4.rounds, 10);
/// assert_eq!(emom4.every.duration, 2);
/// assert_eq!(emom4.alternating, true);
/// assert_eq!(emom4.rest.duration, 0);
/// assert_eq!(emom4.rest.unit, "");
///
/// let emom5: EMOM = "emom-10-r30s-alt".parse().unwrap();
/// assert_eq!(emom5.rounds, 10);
/// assert_eq!(emom5.every.duration, 1);
/// assert_eq!(emom5.alternating, true);
/// assert_eq!(emom5.rest.duration, 30);
/// assert_eq!(emom5.rest.unit, "s");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct EMOM {
    /// The number of rounds to perform the workout.
    pub rounds: u16,
    /// The interval at which the exercise is performed. Defaults to 1 minute.
    pub every: Every,
    /// A boolean indicating whether the workout is alternating between exercises.
    pub alternating: bool,
    /// A `Rest` struct representing the rest period between exercises.
    pub rest: Every,
}

impl FromStr for EMOM {
    /// Creates a new `EMOM` instance from a string representation.
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into number part and name part
        let parts: Vec<&str> = s.split('-').collect();
        let mut alternating = false;
        let mut every = Every {
            duration: 1,
            unit: "m".to_string(),
            rest: false,
        };
        let mut rounds = 1;
        let mut rest = Every {
            duration: 0,
            unit: "".to_string(),
            rest: false,
        };

        let mut counter = 0;
        for part in parts.iter() {
            match *part {
                "emom" => {
                    continue;
                }
                "alt" => {
                    alternating = true;
                }
                _ => {
                    if part.starts_with('r') && (part.contains('m') | part.contains('s')) {
                        rest = Every::from_str(part).expect("Invalid Rest format");
                        continue;
                    }

                    if counter == 0 {
                        rounds = part
                            .parse::<u16>()
                            .map_err(|_| "Invalid number format".to_string())?;
                    } else if counter == 1 {
                        every = part
                            .parse::<Every>()
                            // .parse::<u16>()
                            .map_err(|_| "Invalid 'Every' format".to_string())?;
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
        let mut workout = format!("EMOM {} minutes", self.rounds);
        if self.every.duration != 1 {
            workout.push_str(&format!("\n\n{}", self.every));
            // workout.push_str(&format!("\n\nEvery {} minutes", self.every));
        }
        if self.rest.duration != 0 {
            if self.every.duration == 1 {
                // workout.push_str(&format!("\n\nrest {}", self.rest));
                workout.push_str(&format!("\n\n{}", self.rest));
            } else {
                // workout.push_str(&format!(", rest {}", self.rest));
                workout.push_str(&format!(", {}", self.rest));
            }
        }
        if self.alternating {
            workout.push_str(", alternating");
        }

        write!(formatter, "{}", workout)
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
                every: Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: false,
                rest: Every {
                    duration: 0,
                    unit: "".to_string(),
                    rest: false
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: true,
                rest: Every {
                    duration: 0,
                    unit: "".to_string(),
                    rest: false
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-20-2m").unwrap(),
            EMOM {
                rounds: 20,
                every: Every {
                    duration: 2,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: false,
                rest: Every {
                    duration: 0,
                    unit: "".to_string(),
                    rest: false
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-r2m").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: false,
                rest: Every {
                    duration: 2,
                    unit: "m".to_string(),
                    rest: true
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-2m-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 2,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: true,
                rest: Every {
                    duration: 0,
                    unit: "".to_string(),
                    rest: false
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-r30s").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: false,
                rest: Every {
                    duration: 30,
                    unit: "s".to_string(),
                    rest: true
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-r30s-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                },
                alternating: true,
                rest: Every {
                    duration: 30,
                    unit: "s".to_string(),
                    rest: true
                }
            }
        );
        assert_eq!(
            EMOM::from_str("emom-10-30s-alt").unwrap(),
            EMOM {
                rounds: 10,
                every: Every {
                    duration: 30,
                    unit: "s".to_string(),
                    rest: false
                },
                alternating: true,
                rest: Every {
                    duration: 0,
                    unit: "".to_string(),
                    rest: false
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
                    every: Every {
                        duration: 1,
                        unit: "m".to_string(),
                        rest: false
                    },
                    alternating: false,
                    rest: Every {
                        duration: 0,
                        unit: "".to_string(),
                        rest: true
                    }
                }
            ),
            "EMOM 10 minutes"
        );

        assert_eq!(
            format!("{}", EMOM::from_str("emom-10").unwrap()),
            "EMOM 10 minutes"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-10-alt").unwrap()),
            "EMOM 10 minutes, alternating"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-10-2m").unwrap()),
            "EMOM 10 minutes\n\nwork every 2 minutes"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-10-2m-alt").unwrap()),
            "EMOM 10 minutes\n\nwork every 2 minutes, alternating"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-10-r30s").unwrap()),
            "EMOM 10 minutes\n\nrest 30 seconds"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-10-r30s-alt").unwrap()),
            "EMOM 10 minutes\n\nrest 30 seconds, alternating"
        );
        assert_eq!(
            format!("{}", EMOM::from_str("emom-12-3m-r1m-alt").unwrap()),
            "EMOM 12 minutes\n\nwork every 3 minutes, rest 1 minute, alternating"
        );
        assert_eq!(
            format!("{}", "emom-12-3m-r1m-alt".parse::<EMOM>().unwrap()),
            "EMOM 12 minutes\n\nwork every 3 minutes, rest 1 minute, alternating"
        );
        assert_eq!(
            format!("{}", "emom-5-30s".parse::<EMOM>().unwrap()),
            "EMOM 5 minutes\n\nwork every 30 seconds"
        );
    }
}
