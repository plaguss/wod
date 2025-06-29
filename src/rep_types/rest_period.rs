use std::fmt;
use std::str::FromStr;

/// Represents a rest period with a specified duration and unit.
///
/// # Examples
/// ```
/// use wod::RestPeriod;
///
/// let rest: RestPeriod = "r1m".parse().unwrap();
///
/// assert_eq!(rest.duration, 1);
/// assert_eq!(rest.unit, "m");
/// ```
///
/// # Display
/// The "RestPeriod" struct implements the "Display" trait, which allows it to be formatted as a string.
/// ```
/// use wod::RestPeriod;
///
/// let rest: RestPeriod = "r1m".parse().unwrap();
///
/// assert_eq!(format!("{}", rest), "Rest 1 minute");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct RestPeriod {
    /// The length of the rest period.
    pub duration: u16,
    /// The unit of measurement for the rest period (e.g., "s" for seconds, "m" for minutes).
    pub unit: String,
}

impl FromStr for RestPeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut duration = String::new();
        let mut unit = String::new();

        for c in s.to_string().chars() {
            if c.is_numeric() {
                duration.push(c);
            } else if c == 'r' {
                continue;
            } else {
                unit.push(c);
            }
        }

        if duration.is_empty() || unit.is_empty() {
            return Err(format!("Invalid RestPeriod format: '{}'", s));
        }

        let duration_parsed = duration
            .parse::<u16>()
            .map_err(|e| format!("Invalid duration in RestPeriod '{}': {}", s, e))?;

        Ok(RestPeriod {
            duration: duration_parsed,
            unit,
        })
    }
}

impl fmt::Display for RestPeriod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match self.unit.as_str() {
            "m" => {
                if self.duration != 1 {
                    "minutes"
                } else {
                    "minute"
                }
            }
            "s" => "seconds",
            _ => "minutes",
        };
        write!(formatter, "Rest {} {}", self.duration, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest() {
        assert_eq!(
            RestPeriod::from_str("r1m").unwrap(),
            RestPeriod {
                duration: 1,
                unit: "m".to_string(),
            }
        );
        assert_eq!(
            RestPeriod::from_str("r90s").unwrap(),
            RestPeriod {
                duration: 90,
                unit: "s".to_string(),
            }
        );
    }

    #[test]
    fn test_work_display() {
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 1,
                    unit: "m".to_string(),
                }
            ),
            "Rest 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 2,
                    unit: "m".to_string(),
                }
            ),
            "Rest 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 90,
                    unit: "s".to_string(),
                }
            ),
            "Rest 90 seconds"
        );
    }

    #[test]
    fn test_rest_display() {
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 1,
                    unit: "m".to_string(),
                }
            ),
            "Rest 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 2,
                    unit: "m".to_string(),
                }
            ),
            "Rest 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                RestPeriod {
                    duration: 90,
                    unit: "s".to_string(),
                }
            ),
            "Rest 90 seconds"
        );
    }
}
