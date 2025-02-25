
// use crate::movements::{exists_movement, suggest_closest_movement};
use std::str::FromStr;
use crate::movements::MovementName;


#[derive(Clone, Debug, PartialEq)]
pub struct Movement {
    name: MovementName,
    rep_scheme: String, // This has to be a RepScheme type
    description: String,
}

impl Default for Movement {
    // Create a check for to ensure the value is a valid one,
    // and suggest a similar one otherwise.
    fn default() -> Self {
        // Movement {name: "air squat", rep_scheme: "5x5", description: ""}
        Movement {
            name: MovementName::from_str("air squat").unwrap(),
            rep_scheme: "5x5".to_string(),
            description: "".to_string(),
        }
    }
}

// THE MOVEMENTS ARE ALREADY DEFINED, IT DOESN'T MAKE SENSE TO CREATE THIS NOW
impl Movement {
    pub fn new(name: &str, rep_scheme: &str, description: &str) -> Self {
        let normalized_name = name.to_string().to_lowercase();
        // if !exists_movement(&normalized_name) {
        //     if let Some(closest) = suggest_closest_movement(&normalized_name) {
        //         panic!("Movement `{}` does not exist. Did you mean `{}`?", name, closest);
        //     }
        // }
        if let Some(name) = MovementName::from_str(&normalized_name) {
            return Movement {
                name: name,
                rep_scheme: rep_scheme.to_string(),
                description: description.to_string(),
            }
        }
        let name = MovementName::from_str(&normalized_name);
        Movement {
            name: name,
            // TODO: Create a RepScheme type
            rep_scheme: rep_scheme.to_string(),
            description: description.to_string(),
        }
    }

    // pub fn name(&self) -> &str {
    //     &self.name
    // }

    // pub fn rep_scheme(&self) -> &str {
    //     &self.rep_scheme
    // }

    // pub fn description(&self) -> &str {
    //     &self.description
    // }
}


#[cfg(test)]
mod tests {
    use super::Movement;

    #[test]
    fn test_default() {
        let movement = Movement::default();
        assert_eq!(movement.name, "air squat");
        assert_eq!(movement.rep_scheme, "5x5");
        assert_eq!(movement.description, "");
    }

    #[test]
    fn test_movement() {
        let movement = Movement::new("front squat", "5x5", "Normal squat without apparel");
        assert_eq!(movement.name, "front squat");
        assert_eq!(movement.rep_scheme, "5x5");
        assert_eq!(movement.description, "Normal squat without apparel");
    }
    
    #[test]
    #[should_panic(expected = "Movement `Pushup` does not exist. Did you mean `push up`?")]
    fn test_movement_new_invalid_with_suggestion() {
        Movement::new("Pushup", "3x10", "Pushups with bodyweight");
    }
}