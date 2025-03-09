use std::fmt;
use std::str::FromStr;

use crate::workout_types::amrap::AMRAP;
use crate::workout_types::emom::EMOM;
use crate::workout_types::for_time::ForTime;
use crate::workout_types::rest::Rest;

#[derive(Debug, PartialEq, Clone)]
pub enum WorkoutType {
    ForTime(ForTime),
    AMRAP(AMRAP),
    EMOM(EMOM),
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
        Err("Invalid workout type".to_string())
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
