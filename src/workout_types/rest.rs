use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Rest {
    pub duration: u16,
    pub unit: String,
}

impl FromStr for Rest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut duration = String::new();
        let mut unit = String::new();

        for c in s.chars() {
            if c.is_numeric() {
                duration.push(c);
            } else {
                unit.push(c);
            }
        }
        Ok(Rest {
            duration: duration.parse().unwrap(),
            unit: unit,
        })
    }
}

impl fmt::Display for Rest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = match self.unit.as_str() {
            "m" => if self.duration != 1 {"minutes"} else {"minute"},
            "s" => "seconds",
            _ => "unknown",
        };
        // let mut rest = format!("rest {} {}}", self.rounds, unit);
        // if self.every != 1 {
        //     workout.push_str(&format!("\n\nEvery {} minutes", self.every));
        // }
        // if self.rest.duration != 0 {
        //     workout.push_str(&format!(", {}", self.rest));
        // }
        // if self.alternating {
        //     workout.push_str(", alternating");
        // }
        // workout.push_str("\n\n");
        write!(formatter, "rest {} {}", self.duration, unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rest() {
        assert_eq!(
            Rest::from_str("1m").unwrap(),
            Rest {
                duration: 1,
                unit: "m".to_string()
            }
        );
        assert_eq!(
            Rest::from_str("90s").unwrap(),
            Rest {
                duration: 90,
                unit: "s".to_string()
            }
        );
    }

    #[test]
    fn test_rest_display() {
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 1,
                    unit: "m".to_string()
                }
            ),
            "rest 1 minute"
        );
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 2,
                    unit: "m".to_string()
                }
            ),
            "rest 2 minutes"
        );
        assert_eq!(
            format!(
                "{}",
                Rest {
                    duration: 90,
                    unit: "s".to_string()
                }
            ),
            "rest 90 seconds"
        );
    }
}
