use std::fmt;

// Struct to deal with 70kg, 70%, 60/40kg, etc.
#[derive(Clone, Debug, PartialEq)]
pub struct Weight {
    weight_man: u32,
    weight_woman: u32,
    unit: String,
}

// If a woman's weight is not informed, it will be the same
fn extract_unit(w: &str) -> (u32, u32, String) {
    let mut weight_man = String::new();
    let mut unit = String::new();
    let mut weight_woman = String::new();

    // To deal with one/two weights
    let mut is_man = true;

    for c in w.chars() {
        // Assume the first number is the weight for man
        if c == '/' {
            is_man = false;
            continue;
        }
        if c.is_numeric() {
            match is_man {
                true => weight_man.push(c),
                false => weight_woman.push(c),
            }
        } else {
            unit.push(c);
        }
    }

    // If is_man is true, it means only one weight is informed, then
    // copy the value from the man
    let weight_woman = if is_man {
        weight_man.clone()
    } else {
        weight_woman
    };

    // unwrap and parse to cast the string
    (
        weight_man.parse().unwrap(),
        weight_woman.parse().unwrap(),
        unit,
    )
}

impl Weight {
    pub fn from(w: String) -> Self {
        let (weight_man, weight_woman, unit) = extract_unit(&w);
        Weight {
            weight_man,
            weight_woman,
            unit,
        }
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.weight_woman != self.weight_man {
            write!(
                f,
                "{weight_man}/{weight_woman}{unit}",
                weight_man = self.weight_man,
                weight_woman = self.weight_woman,
                unit = self.unit
            )
        } else {
            write!(f, "{}{}", self.weight_man, self.unit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight() {
        assert_eq!(
            Weight::from("70kg".to_string()),
            Weight {
                weight_man: 70,
                weight_woman: 70,
                unit: "kg".to_string()
            }
        );
        assert_eq!(
            Weight::from("70%".to_string()),
            Weight {
                weight_man: 70,
                weight_woman: 70,
                unit: "%".to_string()
            }
        );
        assert_eq!(
            Weight::from("60/40kg".to_string()),
            Weight {
                weight_man: 60,
                weight_woman: 40,
                unit: "kg".to_string()
            }
        );
    }

    // Add test for the print
    #[test]
    fn test_weight_display() {
        assert_eq!(
            format!("{}", Weight::from("70kg".to_string())),
            "70kg".to_string()
        );
        assert_eq!(
            format!("{}", Weight::from("70%".to_string())),
            "70%".to_string()
        );
        assert_eq!(
            format!("{}", Weight::from("60/40kg".to_string())),
            "60/40kg".to_string()
        );
    }
}
