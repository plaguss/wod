use std::fmt;
use std::str::FromStr;

// EMOMS
#[derive(Debug, PartialEq, Clone)]
pub struct EMOM {
    // The number of minutes to perform the workout
    pub rounds: u16,
    pub every: u16,
    pub alternating: bool,
    pub rest: Rest,
}

impl FromStr for EMOM {
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
        return Ok(EMOM {
            rounds,
            every,
            alternating,
            rest,
        });
    }
}

impl fmt::Display for EMOM {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "EMOM {} minutes", self.rounds)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rest {
    pub duration: u16,
    pub unit: String,
}

impl FromStr for Rest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut duration = String::new();
        let mut unit = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                duration.push(c);
            } else {
                unit.push(c);
            }
        }
        Ok(Rest {
            duration: duration.parse().unwrap(),
            unit: unit,
        })
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
    fn test_rest() {
        assert_eq!(
            Rest::from_str("1m").unwrap(),
            Rest {
                duration: 1,
                unit: "m".to_string()
            }
        );
        assert_eq!(
            Rest::from_str("90s").unwrap(),
            Rest {
                duration: 90,
                unit: "s".to_string()
            }
        );
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
