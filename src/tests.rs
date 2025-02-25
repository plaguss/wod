use crate::lexer::Lexer;
use crate::workout::Workout;
use crate::workout_type::{ForTime, WorkoutType};

#[test]
fn test_workout_parse() {
    let input = "ft 21-15-9 pull up, thruster";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut workout = Workout::default();
    workout.parse(tokens);

    assert_eq!(workout.movements.len(), 2);
    assert_eq!(workout.reps.len(), 3);
    assert_eq!(workout.rep_types.len(), 0);
    assert_eq!(
        workout.workout_type,
        WorkoutType::ForTime(ForTime {
            rounds: 1,
            name: "ft".to_string()
        })
    );
}
