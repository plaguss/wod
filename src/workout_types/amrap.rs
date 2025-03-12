use std::fmt;
use std::str::FromStr;

/// Represents an As Many Reps As Possible (AMRAP) workout.
///
/// This struct is used to define a workout session where the goal is to perform as many repetitions
/// as possible within a specified number of minutes.
///
/// # Examples
///
/// ## Parsing
///
/// The format should be `amrap-<minutes>`, where `<minutes>` is the number of minutes for the AMRAP.
/// If the minutes part is missing or invalid, it defaults to 1 minute.
///
/// ```
/// use wod::AMRAP;
///
/// let amrap: AMRAP = "amrap-10".parse().unwrap();
/// assert_eq!(amrap, AMRAP { minutes: 10 });
///
/// let amrap_default: AMRAP = "amrap-".parse().unwrap();
/// assert_eq!(amrap_default, AMRAP { minutes: 1 });
/// ```
///
/// ## Display
///
/// Formats the `AMRAP` for display.
///
/// The output format is `AMRAP <minutes> minutes`.
///
/// ```
/// use wod::AMRAP;
/// let amrap = AMRAP { minutes: 10 };
/// assert_eq!(format!("{}", amrap), "AMRAP 10 minutes".to_string());
/// ```
#[derive(Debug, PartialEq, Clone)]
pub struct AMRAP {
    /// The number of minutes allocated for the workout.
    pub minutes: u8,
    // TODO: For more complex AMRAPs
    // // The number of sets to perform
    // sets: u32,
    // // The number of minutes to rest between sets
    // rest: u32,
}

impl FromStr for AMRAP {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into number part and name part
        let parts: Vec<&str> = s.split("-").collect();
        if parts[0] != "amrap" {
            return Err("Invalid AMRAP format".to_string());
        }
        if parts.len() != 2 {
            return Err("Invalid AMRAP format".to_string());
        }
        let number_part = parts[1];

        // Parse the time domain
        let minutes = if number_part.is_empty() {
            1 // Default to 1 if no number is present
        } else {
            number_part
                .parse::<u8>()
                .map_err(|_| "Invalid number format".to_string())?
        };

        Ok(AMRAP { minutes })
    }
}

impl fmt::Display for AMRAP {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "AMRAP {} minutes", self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amrap() {
        assert_eq!(AMRAP::from_str("amrap-10").unwrap(), AMRAP { minutes: 10 });
        assert_eq!(AMRAP::from_str("amrap-").unwrap(), AMRAP { minutes: 1 });
    }

    #[test]
    fn test_amrap_invalid() {
        assert!(AMRAP::from_str("amrap").is_err());
        assert!(AMRAP::from_str("other-10").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", AMRAP { minutes: 10 }),
            "AMRAP 10 minutes".to_string()
        );
    }
}
