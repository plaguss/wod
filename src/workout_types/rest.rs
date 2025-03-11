use std::fmt;
use std::str::FromStr;

/// Represents a rest period with a specified duration and unit.
///
/// # Examples
/// ```
/// use wod::Rest;
///
/// let rest: Rest = "1m".parse().unwrap();
///
/// assert_eq!(rest.duration, 1);
/// assert_eq!(rest.unit, "m");
/// ```
///
/// # Display
/// The "Rest" struct implements the "Display" trait, which allows it to be formatted as a string.
/// ```
/// use wod::Rest;
///
/// let rest: Rest = "1m".parse().unwrap();
///
/// assert_eq!(format!("{}", rest), "rest 1 minute");
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct Rest {
    /// The length of the rest period.
    pub duration: u16,
    /// The unit of measurement for the rest period (e.g., "s" for seconds, "m" for minutes).
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

impl fmt::Display for Rest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match self.unit.as_str() {
            "m" => if self.duration != 1 {"minutes"} else {"minute"},
            "s" => "seconds",
            _ => "unknown",
        };
        write!(formatter, "rest {} {}", self.duration, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_rest_display() {
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 1,
                    unit: "m".to_string()
                }
            ),
            "rest 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 2,
                    unit: "m".to_string()
                }
            ),
            "rest 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 90,
                    unit: "s".to_string()
                }
            ),
            "rest 90 seconds"
        );
    }
}
