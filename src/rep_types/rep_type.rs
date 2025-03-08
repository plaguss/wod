use std::fmt;
use std::str::FromStr;

use crate::rep_types::distance::Distance;


// TODO: All of these must take into account men/woman, so 30/20 cals, 20/15 (for reps),
// 400/300m, etc. Use Weight as reference
#[derive(Debug, PartialEq, Clone)]
pub enum RepType {
    // Default number of repetitions, e.g. 10 or whatever single number
    Reps(u16),
    // Distance, e.g. 100m, 5K
    Distance(Distance),
    // Calories, e.g. 10cals
    Cals(u16),
}

impl FromStr for RepType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Check if it's a distance, e.g. 100m, 5K
        if s.ends_with("m") || s.to_lowercase().ends_with("k") {
            return Ok(RepType::Distance(Distance::from(s.to_string())));
        }

        // Check if it's a number followed by "cals"
        if s.contains("cal") {
            let parts: Vec<&str> = s.split("cal").collect();
            if parts.len() != 2 {
                return Err("Invalid rep type".to_string());
            }
            let cals = parts[0]
                .parse::<u16>()
                .map_err(|_| "Invalid number format".to_string())?;
            return Ok(RepType::Cals(cals));
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
            RepType::Cals(cals) => write!(formatter, "{} calories", cals),
            RepType::Distance(distance) => write!(formatter, "{}", distance),
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
            RepType::Distance(Distance::from("100m".to_string()))
        );
        assert_eq!(
            RepType::from_str("5k").unwrap(),
            RepType::Distance(Distance::from("5k".to_string()))
        );
        assert_eq!(RepType::from_str("10cals").unwrap(), RepType::Cals(10));
    }
}
