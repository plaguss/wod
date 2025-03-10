use std::fmt;
use std::str::FromStr;

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

impl FromStr for RM {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RM { num: extract_rm(s) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rm() {
        assert_eq!(RM::from_str("1rm").unwrap(), RM { num: 1 });
        assert_eq!(RM::from_str("3rm").unwrap(), RM { num: 3 });
        assert_eq!(RM::from_str("5rm").unwrap(), RM { num: 5 });
    }
}
