#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::workout::{create_workout, Workout};
    use crate::{ForTime, WorkoutType};

    //TODO: Start from here until a decent amount of wods can be represented as strings
    // The workouts are too simple yet.
    // - They need to accomodate weights
    // - The weights have to be associated to the corresponding movement
    // - Prepare the workouts to determine where everything has to be placed
    #[test]
    fn test_for_time() {
        let workout = create_workout("ft 21-15-9 pull up, thruster @43/30kg");

        assert_eq!(workout.movements.len(), 2);
        assert_eq!(workout.rep_types.len(), 3);
        assert_eq!(
            workout.workout_type,
            WorkoutType::ForTime(ForTime {
                rounds: 1,
                name: "ft".to_string()
            })
        );
        let expected = "**For Time**\n21-15-9\n\n- Pull Up\n\n- Thruster At 43/30Kg\n\n";
        assert_eq!(workout.write(), expected);

    }

    // For weightlifting a small hint should be placed for what (4x2) means (Low priority)
    #[test]
    fn test_weightlifting_0() {
        let workout = create_workout("wl 4x2 @ 85% snatch");
        assert_eq!(workout.movements.len(), 1);
        assert_eq!(workout.rep_types.len(), 2);
        assert_eq!(workout.workout_type, WorkoutType::Weightlifting);
        let expected = "**Weightlifting**\n\n4x2 @ 85% snatch\n\n";
        assert_eq!(workout.write(), expected);

    }
}
