use std::fmt;
use std::str::FromStr;

use crate::rep_types::{cals::Cals, distance::Distance};

/// TODO: All of these must take into account men/woman, so 30/20 cals, 20/15 (for reps),
/// Represents different types of repetitions or measures in a workout.
///
/// This enum can be used to specify the type of repetition, distance, calories, or
/// maximum reps in a workout routine.
///
/// # Examples
///
/// ```
/// use wod::{RepType, Distance};
///
/// let reps = "10".parse::<RepType>().unwrap();
/// assert_eq!(reps, RepType::Reps(10));
/// let distance = RepType::Distance("100m".parse::<Distance>().unwrap());
/// assert_eq!(distance, RepType::Distance("100m".parse::<Distance>().unwrap()));
/// let cals = "10cals".parse::<RepType>().unwrap();
/// assert_eq!(cals, RepType::Cals(10));
/// let max = "max".parse::<RepType>().unwrap();
/// assert_eq!(max, RepType::Max);
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum RepType {
    /// Default number of repetitions, e.g. 10 or whatever single number
    Reps(u16),
    /// Distance, e.g. 100m, 5K
    Distance(Distance),
    /// Calories, e.g. 10cal, 100/80cal
    Cals(Cals),
    /// Max reps of a given movement in a time.
    Max,
}

impl FromStr for RepType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if it's a distance, e.g. 100m, 5K
        if s.ends_with('m') || s.to_lowercase().ends_with('k') {
            return Ok(RepType::Distance(
                s.parse::<Distance>().expect("Invalid distance"),
            ));
        }

        // Check if it's a number followed by "cal"
        if s.contains("cal") {
            return Ok(RepType::Cals(s.parse::<Cals>().expect("Invalid calories")));
        }

        if s == "max" {
            return Ok(RepType::Max);
        }

        // Check if it's a number
        if let Ok(reps) = s.parse::<u16>() {
            return Ok(RepType::Reps(reps));
        }

        Err("Invalid rep type".to_string())
    }
}

impl fmt::Display for RepType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepType::Reps(reps) => write!(formatter, "{}", reps),
            RepType::Cals(cals) => write!(formatter, "{}", cals),
            RepType::Distance(distance) => write!(formatter, "{}", distance),
            RepType::Max => write!(formatter, "Max reps of"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rep_type_from_str() {
        assert_eq!(RepType::from_str("10").unwrap(), RepType::Reps(10));
        assert_eq!(
            RepType::from_str("100m").unwrap(),
            RepType::Distance("100m".parse::<Distance>().unwrap())
        );
        assert_eq!(
            RepType::from_str("5k").unwrap(),
            RepType::Distance("5k".parse::<Distance>().unwrap())
        );
        // assert_eq!(RepType::from_str("10cals").unwrap(), RepType::Cals(10));
        assert_eq!(
            RepType::from_str("10cal").unwrap(),
            RepType::Cals("10cal".parse::<Cals>().unwrap())
        );
        assert_eq!(RepType::from_str("max").unwrap(), RepType::Max);
    }

    #[test]
    fn test_rep_type_display() {
        assert_eq!(format!("{}", RepType::Reps(10)), "10".to_string());
        assert_eq!(
            format!("{}", RepType::Distance("100m".parse().unwrap())),
            "100m".to_string()
        );
        assert_eq!(
            format!("{}", RepType::from_str("10cal").unwrap()),
            "10 calories".to_string()
        );
        assert_eq!(format!("{}", RepType::Max), "Max reps of".to_string());
    }
}
