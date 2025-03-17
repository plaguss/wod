#[cfg(test)]
mod testing {
    use crate::workout::create_workout;
    //TODO: Start from here until a decent amount of wods can be represented as strings
    // The workouts are too simple yet.
    // - They need to accomodate weights
    // - The weights have to be associated to the corresponding movement
    // - Prepare the workouts to determine where everything has to be placed
    #[test]
    fn test_for_time_0() {
        let workout = create_workout("ft 21-15-9 pull up, thruster @43/30kg").unwrap();
        let expected = "---\n\n**For Time**\n\n21-15-9\n\n- Pull Up\n\n- Thruster At 43/30kg\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_for_time_1() {
        let workout = create_workout("4rd 21 box jump over, 15 bar mu").unwrap();
        let expected =
            "---\n\n**4 rounds for time**\n\n- 21 Box Jump Over\n\n- 15 Bar Muscle Up\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_for_time_2() {
        let workout =
            create_workout("3rd 15 chest to bar, 15cal echo bike, 15 thruster @40kg").unwrap();
        let expected = "---\n\n**3 rounds for time**\n\n- 15 Chest To Bar\n\n- 15 calories Echo Bike\n\n- 15 Thruster At 40kg\n\n";
        assert_eq!(workout.write(), expected);
    }

    // For weightlifting a small hint should be placed for what (4x2) means (Low priority)
    #[test]
    fn test_weightlifting_0() {
        let workout = create_workout("wl 4x2 snatch @ 85%").unwrap();
        let expected = "---\n\n**Weightlifting**\n\n4x2 Snatch @ 85%\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_weightlifting_1() {
        let workout = create_workout("wl 3x(1+1+1) clean,front squat,split jerk @ 80kg").unwrap();
        let expected =
            "---\n\n**Weightlifting**\n\n3x(1+1+1) Clean + Front Squat + Split Jerk @ 80kg\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_weightlifting_2() {
        let workout = create_workout("wl 3x(1+1) clean,split jerk @ 80kg").unwrap();
        let expected = "---\n\n**Weightlifting**\n\n3x(1+1) Clean + Split Jerk @ 80kg\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_emom_1() {
        let workout =
            create_workout("emom-12 15cal row, 12 toes to bar, max db clean and jerk @ 22/15kg")
                .unwrap();
        let expected = "---\n\n**EMOM 12 minutes**\n\n- 15 calories Row\n\n- 12 Toes To Bar\n\n- Max reps of Dumbbell Clean and Jerk At 22/15kg\n\n";
        assert_eq!(workout.write(), expected);
    }

    #[test]
    fn test_emom_2() {
        let workout = create_workout(
            "emom-12-3m-r1m 15cal row, 12 toes to bar, max db clean and jerk @ 22/15kg",
        )
        .unwrap();
        let expected = "---\n\n**EMOM 12 minutes**\n\nwork every 3 minutes, rest 1 minute\n\n- 15 calories Row\n\n- 12 Toes To Bar\n\n- Max reps of Dumbbell Clean and Jerk At 22/15kg\n\n";
        assert_eq!(workout.write(), expected);
    }
}
