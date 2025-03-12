use std::fmt;
use std::str::FromStr;

use crate::rep_types::split_gender_unit;

/// Represents calories for both men and women.
///
/// # Examples
///
/// ## Creating a "Cals" instance
///
/// You can create a "Cals" instance by parsing a string.
///
/// ```
/// use wod::Cals;
///
/// let cals: Cals = "100cal".parse().unwrap();
/// assert_eq!(
///     cals,
///     Cals {
///        cals_man: 100,
///        cals_woman: 100,
///     }
/// );
/// ```
///
/// ## Displaying a "Cals" instance
///
/// The "Cals" will be displayed as is.
///
/// ```
/// use wod::Cals;
///
/// let cals: Cals = "100cal".parse().unwrap();
/// assert_eq!(format!("{}", cals), "100 calories".to_string());
/// let cals_mf: Cals = "100/80cal".parse().unwrap();
/// assert_eq!(format!("{}", cals_mf), "100/80 calories".to_string());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Cals {
    /// Calories for men.
    pub cals_man: u32,
    /// Calories for women.
    pub cals_woman: u32,
}

impl FromStr for Cals {
    type Err = String;
    fn from_str(w: &str) -> Result<Self, Self::Err> {
        let (cals_man, cals_woman, _unit) = split_gender_unit(w);
        Ok(Cals {
            cals_man,
            cals_woman,
        })
    }
}

impl fmt::Display for Cals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cals_woman != self.cals_man {
            write!(
                f,
                "{man}/{woman} calories",
                man = self.cals_man,
                woman = self.cals_woman,
            )
        } else {
            write!(f, "{} calories", self.cals_man)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cals() {
        assert_eq!(
            "100cal".parse::<Cals>().unwrap(),
            Cals {
                cals_man: 100,
                cals_woman: 100,
            }
        );
        assert_eq!(
            "100/80cal".parse::<Cals>().unwrap(),
            Cals {
                cals_man: 100,
                cals_woman: 80,
            }
        );
    }

    #[test]
    fn test_cals_display() {
        assert_eq!(
            format!("{}", "100cal".parse::<Cals>().unwrap()),
            "100 calories".to_string()
        );
        assert_eq!(
            format!("{}", "100/80cal".parse::<Cals>().unwrap()),
            "100/80 calories".to_string()
        );
    }
}
