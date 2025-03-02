use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct EMOM {
    // The number of minutes to perform the workout
    pub minutes: u32,
}

impl FromStr for EMOM {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into number part and name part
        let parts: Vec<&str> = s.split("-").collect();
        if parts[0] != "emom" {
            return Err("Invalid EMOM format".to_string());
        }
        if parts.len() != 2 {
            return Err("Invalid EMOM format".to_string());
        }
        let number_part = parts[1];

        // Parse the time domain
        let minutes = if number_part.is_empty() {
            1 // Default to 1 if no number is present
        } else {
            number_part
                .parse::<u32>()
                .map_err(|_| "Invalid number format".to_string())?
        };

        Ok(EMOM { minutes })
    }
}

impl fmt::Display for EMOM {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "EMOM {} minutes", self.minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emom() {
        assert_eq!(EMOM::from_str("emom-10").unwrap(), EMOM { minutes: 10 });
        assert_eq!(EMOM::from_str("emom-").unwrap(), EMOM { minutes: 1 });
    }

    #[test]
    fn test_emom_invalid() {
        assert!(EMOM::from_str("emom").is_err());
        assert!(EMOM::from_str("other-10").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", EMOM { minutes: 10 }), "EMOM 10 minutes");
    }
}
