use std::fmt;

// Struct to deal with 1rm, 3rm, etc.
#[derive(Clone, Debug, PartialEq)]
pub struct RM {
    pub num: u8,
}

fn extract_rm(m: &str) -> u8 {
    let mut num = String::new();
    for c in m.chars() {
        if c.is_numeric() {
            num.push(c);
        }
    }
    num.parse().unwrap()
}

impl RM {
    pub fn from(movement: String) -> Self {
        RM {
            num: extract_rm(&movement),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rm() {
        assert_eq!(RM::from("1rm".to_string()), RM { num: 1 });
        assert_eq!(RM::from("3rm".to_string()), RM { num: 3 });
        assert_eq!(RM::from("5rm".to_string()), RM { num: 5 });
    }
}
