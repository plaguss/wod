use std::fmt;
use std::str::FromStr;

use crate::workout_types::{amrap::AMRAP, emom::EMOM, for_time::ForTime};

/// Represents different types of workouts.
///
/// This enum categorizes workouts into four main types:
/// - `ForTime`: A workout that is completed as fast as possible.
/// - `AMRAP`: As Many Rounds As Possible within a set time.
/// - `EMOM`: Every Minute On the Minute, typically involving a specific exercise or set of exercises.
/// - `Weightlifting`: Focused on weightlifting exercises.
///
/// # Examples
///
/// ```
/// use wod::{WorkoutType, ForTime, AMRAP, EMOM};
///
/// let for_time_workout = WorkoutType::ForTime(ForTime { rounds: 1, name: "ft".to_string() });
/// let amrap_workout = WorkoutType::AMRAP(AMRAP { minutes: 10 });
/// let emom_workout = WorkoutType::EMOM("emom-10".parse::<EMOM>().unwrap());
/// let weightlifting_workout = WorkoutType::Weightlifting;
/// ```
#[derive(Debug, PartialEq, Clone)]
pub enum WorkoutType {
    /// Represents a `ForTime` workout.
    ForTime(ForTime),
    /// Represents an `AMRAP` (As Many Rounds As Possible) workout.
    AMRAP(AMRAP),
    /// Represents an `EMOM` (Every Minute On the Minute) workout.
    EMOM(EMOM),
    /// Represents a Weightlifting workout.
    Weightlifting,
}

impl FromStr for WorkoutType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("amrap") {
            return AMRAP::from_str(s).map(WorkoutType::AMRAP);
        }
        if s.starts_with("emom") {
            return EMOM::from_str(s).map(WorkoutType::EMOM);
        }
        if s == "wl" {
            return Ok(WorkoutType::Weightlifting);
        }
        if s == "ft" || s.contains("rd") {
            // ft, 5rd, 2rd, etc.
            return ForTime::from_str(s).map(WorkoutType::ForTime);
        }
        Err(format!("Invalid workout type: {}", s))
    }
}

impl fmt::Display for WorkoutType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkoutType::ForTime(ft) => write!(formatter, "{}", ft),
            WorkoutType::AMRAP(amrap) => write!(formatter, "{}", amrap),
            WorkoutType::EMOM(emom) => write!(formatter, "{}", emom),
            WorkoutType::Weightlifting => write!(formatter, "Weightlifting"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workout_type_from_str() {
        assert_eq!(
            WorkoutType::from_str("ft").unwrap(),
            WorkoutType::ForTime(ForTime {
                rounds: 1,
                name: "ft".to_string()
            })
        );
        assert_eq!(
            WorkoutType::from_str("amrap-10").unwrap(),
            WorkoutType::AMRAP(AMRAP { minutes: 10 })
        );
        assert_eq!(
            WorkoutType::from_str("emom-10").unwrap(),
            WorkoutType::EMOM(EMOM::from_str("emom-10").unwrap())
        );
        assert_eq!(
            "wl".parse::<WorkoutType>().unwrap(),
            WorkoutType::Weightlifting
        );
    }

    #[test]
    fn test_workout_type_display() {
        assert_eq!(
            format!(
                "{}",
                WorkoutType::ForTime(ForTime {
                    rounds: 1,
                    name: "ft".to_string()
                })
            ),
            "For Time"
        );
        assert_eq!(
            format!("{}", WorkoutType::AMRAP(AMRAP { minutes: 10 })),
            "AMRAP 10 minutes"
        );
        assert_eq!(
            format!("{}", WorkoutType::EMOM(EMOM::from_str("emom-10").unwrap())),
            "EMOM 10 minutes"
        );
        assert_eq!(format!("{}", WorkoutType::Weightlifting), "Weightlifting");
    }
}
