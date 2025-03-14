use std::fmt;
use std::str::FromStr;

use crate::rep_types::split_gender_unit;

/// Represents calories for both men and women.
///
/// # Examples
///
/// ## Creating a "Reps" instance
///
/// You can create a "Reps" instance by parsing a string.
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
pub struct Reps {
    /// Calories for men.
    pub reps_man: u32,
    /// Calories for women.
    pub reps_woman: u32,
}

impl FromStr for Reps {
    type Err = String;
    fn from_str(w: &str) -> Result<Self, Self::Err> {
        let (reps_man, reps_woman, _unit) = split_gender_unit(w);
        Ok(Reps {
            reps_man,
            reps_woman,
        })
    }
}

impl fmt::Display for Reps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.reps_woman != self.reps_man {
            write!(
                f,
                "{man}/{woman}",
                man = self.reps_man,
                woman = self.reps_woman,
            )
        } else {
            write!(f, "{}", self.reps_man)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reps() {
        assert_eq!(
            "100".parse::<Reps>().unwrap(),
            Reps {
                reps_man: 100,
                reps_woman: 100,
            }
        );
        assert_eq!(
            "100/80".parse::<Reps>().unwrap(),
            Reps {
                reps_man: 100,
                reps_woman: 80,
            }
        );
    }

    #[test]
    fn test_reps_display() {
        assert_eq!(
            format!("{}", "100".parse::<Reps>().unwrap()),
            "100".to_string()
        );
        assert_eq!(
            format!("{}", "100/80".parse::<Reps>().unwrap()),
            "100/80".to_string()
        );
    }
}
