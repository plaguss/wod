use std::fmt;

// Struct to deal with 10m, 5K, etc.
#[derive(Clone, Debug, PartialEq)]
pub struct Distance {
    pub num: u32,
    pub unit: String,
}

fn extract_distance(d: &str) -> (u32, String) {
    let mut num = String::new();
    let mut unit = String::new();

    for c in d.chars() {
        if c.is_numeric() {
            num.push(c);
        } else {
            unit.push(c);
        }
    }

    (num.parse().unwrap(), unit)
}

impl Distance {
    pub fn from(d: String) -> Self {
        let (num, unit) = extract_distance(&d);
        Distance { num, unit }
    }
}

impl fmt::Display for Distance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.num, self.unit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(
            Distance::from("100m".to_string()),
            Distance {
                num: 100,
                unit: "m".to_string()
            }
        );
        assert_eq!(
            Distance::from("5k".to_string()),
            Distance {
                num: 5,
                unit: "k".to_string()
            }
        );
        assert_eq!(
            Distance::from("5K".to_string()),
            Distance {
                num: 5,
                unit: "K".to_string()
            }
        );
        assert_eq!(
            Distance::from("1mile".to_string()),
            Distance {
                num: 1,
                unit: "mile".to_string()
            }
        );
    }
}
