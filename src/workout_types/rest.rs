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
}
