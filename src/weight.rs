use std::fmt;
use std::str::FromStr;

/// Represents weight information for both men and women, along with the unit of measurement.
///
/// # Examples
///
/// ## Creating a `Weight` instance
///
/// You can create a `Weight` instance by parsing a string using the `from_str` method.
///
/// ```
/// use wod::weight::Weight;
///
/// let weight = Weight::from_str("70kg").unwrap();
/// assert_eq!(weight, Weight {
///     weight_man: 70,
///     weight_woman: 70,
///     unit: "kg".to_string(),
/// });
/// ```
///
/// Or inderectly by parsing the string.
///
/// ```
/// use wod::weight::Weight;
/// let weight: Weight = "70kg".parse().unwrap();
/// assert_eq!(
///     weight,
///     Weight {
///        weight_man: 70,
///        weight_woman: 70,
///        unit: "kg".to_string()
///     }
/// );
///
///
/// ## Displaying a `Weight` instance
///
/// The `Weight` will be displayed as is.
///
/// ```
/// use wod::weight::Weight;
///
/// let weight = Weight::from_str("70kg").unwrap();
/// assert_eq!(format!("{}", weight), "70kg".to_string());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Weight {
    /// Weight for men.
    pub weight_man: u32,
    /// Weight for women.
    pub weight_woman: u32,
    /// Unit of measurement (e.g., "kg", "lbs").
    pub unit: String,
}

// If a woman's weight is not informed, it will be the same
fn extract_unit(w: &str) -> (u32, u32, String) {
    let mut weight_man = String::new();
    let mut unit = String::new();
    let mut weight_woman = String::new();

    // To deal with one/two weights
    let mut is_man = true;

    for c in w.chars() {
        // Assume the first number is the weight for man
        if c == '/' {
            is_man = false;
            continue;
        }
        if c.is_numeric() {
            match is_man {
                true => weight_man.push(c),
                false => weight_woman.push(c),
            }
        } else {
            unit.push(c);
        }
    }

    // If is_man is true, it means only one weight is informed, then
    // copy the value from the man
    let weight_woman = if is_man {
        weight_man.clone()
    } else {
        weight_woman
    };

    (
        weight_man.parse().unwrap(),
        weight_woman.parse().unwrap(),
        unit,
    )
}

impl FromStr for Weight {
    type Err = String;
    fn from_str(w: &str) -> Result<Self, Self::Err> {
        let (weight_man, weight_woman, unit) = extract_unit(&w);
        Ok(Weight {
            weight_man,
            weight_woman,
            unit,
        })
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.weight_woman != self.weight_man {
            write!(
                f,
                "{weight_man}/{weight_woman}{unit}",
                weight_man = self.weight_man,
                weight_woman = self.weight_woman,
                unit = self.unit
            )
        } else {
            write!(f, "{}{}", self.weight_man, self.unit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight() {
        assert_eq!(
            Weight::from_str("70kg").unwrap(),
            Weight {
                weight_man: 70,
                weight_woman: 70,
                unit: "kg".to_string()
            }
        );
        assert_eq!(
            Weight::from_str("70%").unwrap(),
            Weight {
                weight_man: 70,
                weight_woman: 70,
                unit: "%".to_string()
            }
        );
        assert_eq!(
            Weight::from_str("60/40kg").unwrap(),
            Weight {
                weight_man: 60,
                weight_woman: 40,
                unit: "kg".to_string()
            }
        );
    }

    // Add test for the print
    #[test]
    fn test_weight_display() {
        assert_eq!(
            format!("{}", Weight::from_str("70kg").unwrap()),
            "70kg".to_string()
        );
        assert_eq!(
            format!("{}", Weight::from_str("70%").unwrap()),
            "70%".to_string()
        );
        assert_eq!(
            format!("{}", Weight::from_str("60/40kg").unwrap()),
            "60/40kg".to_string()
        );
    }

    #[test]
    fn test_parse() {
        let weight: Weight = "70kg".parse().unwrap();
        assert_eq!(
            weight,
            Weight {
                weight_man: 70,
                weight_woman: 70,
                unit: "kg".to_string()
            }
        );
    }
}
