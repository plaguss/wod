pub mod cals;
pub mod distance;
pub mod rep_type;

pub fn split_gender_unit(w: &str) -> (u32, u32, String) {
    let mut man = String::new();
    let mut unit = String::new();
    let mut woman = String::new();

    // To deal with one/two units
    let mut is_man = true;

    for c in w.chars() {
        // Assume the first number is the weight for man
        if c == '/' {
            is_man = false;
            continue;
        }
        if c.is_numeric() {
            match is_man {
                true => man.push(c),
                false => woman.push(c),
            }
        } else {
            unit.push(c);
        }
    }

    // If is_man is true, it means only one value is informed, then
    // copy the value from the man
    let woman = if is_man { man.clone() } else { woman };

    (man.parse().unwrap(), woman.parse().unwrap(), unit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_gender_unit() {
        assert_eq!(split_gender_unit("100cal"), (100, 100, "cal".to_string()));
        assert_eq!(split_gender_unit("100/80cal"), (100, 80, "cal".to_string()));
        assert_eq!(split_gender_unit("100"), (100, 100, "".to_string()));
        assert_eq!(split_gender_unit("15/20"), (15, 20, "".to_string()));
    }
}
