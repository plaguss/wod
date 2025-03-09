#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::workout::{Workout, create_workout};
    use crate::{ForTime, WorkoutType};

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
    }

    #[test]
    fn test_weightlifting_0() {
//        let workout = create_workout("wl 4x2 @ 85% snatch-deadlift");
        let workout = create_workout("wl 4x2 @ 85% snatch");
        assert_eq!(workout.movements.len(), 1);
        assert_eq!(workout.rep_types.len(), 2);
        assert_eq!(
            workout.workout_type,
            WorkoutType::Weightlifting
        );
    }

}
