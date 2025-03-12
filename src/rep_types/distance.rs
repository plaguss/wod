use std::fmt;
use std::str::FromStr;

/// Represents a distance with a numeric value and a unit.
///
/// # Examples
///
/// ## Parsing
///
/// The `Distance` struct can be parsed from a string using the `FromStr` trait. The string should be in the format of a number followed by the unit.
///
/// ```
/// use wod::Distance;
///
/// let distance1: Distance = "100m".parse().unwrap();
/// assert_eq!(distance1.num, 100);
/// assert_eq!(distance1.unit, "m".to_string());
///
/// let distance2: Distance = "5k".parse().unwrap();
/// assert_eq!(distance2.num, 5);
/// assert_eq!(distance2.unit, "k".to_string());
///
/// let distance3: Distance = "5K".parse().unwrap();
/// assert_eq!(distance3.num, 5);
/// assert_eq!(distance3.unit, "K".to_string());
///
/// let distance4: Distance = "1mile".parse().unwrap();
/// assert_eq!(distance4.num, 1);
/// assert_eq!(distance4.unit, "mile".to_string());
/// ```
///
/// ## Display
///
/// The `Distance` struct implements the `Display` trait, which allows it to be formatted as a string in the form of `"{number}{unit}"`.
/// ```
/// use wod::Distance;
///
/// let distance: Distance = "100m".parse().unwrap();
/// assert_eq!(format!("{}", distance), "100m".to_string());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Distance {
    /// The numeric value of the distance.
    pub num: u32,
    /// The unit of the distance as a string.(e.g., "m" for meters, "k" for kilometers, "mile" for miles).
    pub unit: String,
}

fn extract_distance(d: &str) -> (u32, String) {
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

impl FromStr for Distance {
    type Err = String;
    fn from_str(d: &str) -> Result<Self, Self::Err> {
        let (num, unit) = extract_distance(&d);
        Ok(Distance { num, unit })
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.num, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            "100m".parse::<Distance>().unwrap(),
            Distance {
                num: 100,
                unit: "m".to_string()
            }
        );
        assert_eq!(
            "5k".parse::<Distance>().unwrap(),
            Distance {
                num: 5,
                unit: "k".to_string()
            }
        );
        assert_eq!(
            "5K".parse::<Distance>().unwrap(),
            Distance {
                num: 5,
                unit: "K".to_string()
            }
        );
        assert_eq!(
            "1mile".parse::<Distance>().unwrap(),
            Distance {
                num: 1,
                unit: "mile".to_string()
            }
        );
    }

    #[test]
    fn test_display() {
        let distance: Distance = "100m".parse().unwrap();
        assert_eq!(format!("{}", distance), "100m".to_string());
    }
}
