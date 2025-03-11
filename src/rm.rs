use std::fmt;
use std::str::FromStr;


/// Represents a "Repetition Maximum" for a weightlifting movement.
///
/// # Examples
///
/// ## Creating a "RM" instance
///
/// You can create a "RM instance by parsing a string.
///
/// ```
/// use wod::rm::RM;
///
/// let rm: RM = "1rm".parse().unwrap();
/// assert_eq!(rm, RM { num: 1});
/// ```
///
/// ## Displaying a "RM" instance
///
/// The "RM" will be displayed as is.
///
/// ```
/// use wod::rm::RM;
///
/// let rm: RM = "1rm".parse().unwrap();
/// assert_eq!(format!("{}", rm), "1rm".to_string());
/// ```
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

impl fmt::Display for RM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{unit}rm", unit = self.num)
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

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", "1rm".parse::<RM>().unwrap()), "1rm");
        assert_eq!(format!("{}", "3rm".parse::<RM>().unwrap()), "3rm");
    }
}
