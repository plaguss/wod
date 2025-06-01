use std::fmt;
use std::str::FromStr;

/// Represents a chunk of time for a movement to be held.
///
/// # Examples
///
/// ## Parsing
///
/// The `Time` struct can be parsed from a string using the `FromStr` trait. The string should be in the format of a number followed by the unit.
///
/// ```
/// use wod::Time;
///
/// let time1: Time = "90sec".parse().unwrap();
/// assert_eq!(time1.num, 90);
/// assert_eq!(time1.unit, "sec".to_string());
///
/// let time2: Time = "1min".parse().unwrap();
/// assert_eq!(time2.num, 1);
/// assert_eq!(time2.unit, "min".to_string());
/// ```
///
/// ## Display
///
/// The `Time` struct implements the `Display` trait, which allows it to be formatted as a string in the form of `"{number}{unit}"`.
/// ```
/// use wod::Time;
///
/// let time1: Time = "90sec".parse().unwrap();
/// assert_eq!(format!("{}", time1), "90 sec".to_string());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Time {
    /// The numeric value of the time.
    pub num: u8,
    /// The unit of the time as a string.(e.g., "sec" for seconds, "min" for minutes).
    pub unit: String,
}

fn extract_time(d: &str) -> (u8, String) {
    let mut num = String::new();
    let mut unit = String::new();

    for c in d.chars() {
        if c.is_numeric() {
            num.push(c);
        } else {
            unit.push(c);
        }
    }

    (num.parse().unwrap(), unit)
}

impl FromStr for Time {
    type Err = String;
    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let (num, unit) = extract_time(d);
        Ok(Time { num, unit })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.num, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        assert_eq!(
            "90sec".parse::<Time>().unwrap(),
            Time {
                num: 90,
                unit: "sec".to_string()
            }
        );
        assert_eq!(
            "1min".parse::<Time>().unwrap(),
            Time {
                num: 1,
                unit: "min".to_string()
            }
        );
    }

    #[test]
    fn test_display() {
        let time: Time = "90sec".parse().unwrap();
        assert_eq!(format!("{}", time), "90 sec".to_string());
    }
}
