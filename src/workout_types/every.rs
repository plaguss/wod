use std::fmt;
use std::str::FromStr;

/// Represents a rest period with a specified duration and unit.
///
/// # Examples
/// ```
/// use wod::Every;
///
/// let rest: Every = "1m".parse().unwrap();
///
/// assert_eq!(rest.duration, 1);
/// assert_eq!(rest.unit, "m");
/// assert_eq!(rest.rest, false);
/// ```
///
/// # Display
/// The "Every" struct implements the "Display" trait, which allows it to be formatted as a string.
/// ```
/// use wod::Every;
///
/// let rest: Every = "r1m".parse().unwrap();
///
/// assert_eq!(format!("{}", rest), "rest 1 minute");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Every {
    /// The length of the rest period.
    pub duration: u16,
    /// The unit of measurement for the rest period (e.g., "s" for seconds, "m" for minutes).
    pub unit: String,
    /// Whether Is used for resting
    pub rest: bool,
}

impl FromStr for Every {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut duration = String::new();
        let mut unit = String::new();
        let mut content = s.to_string();
        let rest = match s.chars().next() {
            Some('r') => {
                content = content.replace('r', "");
                true
            }
            _ => false,
        };

        for c in content.chars() {
            if c.is_numeric() {
                duration.push(c);
            } else {
                unit.push(c);
            }
        }
        Ok(Every {
            duration: duration.parse().unwrap(),
            unit,
            rest,
        })
    }
}

impl fmt::Display for Every {
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
        let maybe_rest = if self.rest { "rest " } else { "work every " };
        write!(formatter, "{}{} {}", maybe_rest, self.duration, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest() {
        assert_eq!(
            Every::from_str("1m").unwrap(),
            Every {
                duration: 1,
                unit: "m".to_string(),
                rest: false
            }
        );
        assert_eq!(
            Every::from_str("90s").unwrap(),
            Every {
                duration: 90,
                unit: "s".to_string(),
                rest: false
            }
        );
        assert_eq!(
            Every::from_str("r1m").unwrap(),
            Every {
                duration: 1,
                unit: "m".to_string(),
                rest: true
            }
        );
        assert_eq!(
            Every::from_str("r90s").unwrap(),
            Every {
                duration: 90,
                unit: "s".to_string(),
                rest: true
            }
        );
    }

    #[test]
    fn test_work_display() {
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: false
                }
            ),
            "work every 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 2,
                    unit: "m".to_string(),
                    rest: false
                }
            ),
            "work every 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 90,
                    unit: "s".to_string(),
                    rest: false
                }
            ),
            "work every 90 seconds"
        );
    }

    #[test]
    fn test_rest_display() {
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 1,
                    unit: "m".to_string(),
                    rest: true
                }
            ),
            "rest 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 2,
                    unit: "m".to_string(),
                    rest: true
                }
            ),
            "rest 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                Every {
                    duration: 90,
                    unit: "s".to_string(),
                    rest: true
                }
            ),
            "rest 90 seconds"
        );
    }
}
