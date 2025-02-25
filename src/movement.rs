use std::fmt;
/// A list of the movements that can be performed.
/// For reference: https://www.crossfit.com/crossfit-movements
use std::str::FromStr;
use strsim::levenshtein;

// TODO: Move this to an Enum, try to implement a FromStr trait
// Display and different checks for the movement creation.
static MOVEMENTS: &[&str] = &[
    // Squat movements
    "air squat",
    "front squat",
    "back squat",
    "overhead squat",
    "pistol squat",
    "goblet squat",
    // Deadlift movements
    "deadlift",
    "sumo deadlift",
    "romanian deadlift",
    // Press movements
    "shoulder press",
    "push press",
    "push jerk",
    "split jerk",
    "bench press",
    // Weightlifting movements
    "clean",
    "power clean",
    "hang clean",
    "hang power clean",
    "clean and jerk",
    "power clean and jerk",
    "clean pull",
    "snatch",
    "power snatch",
    "hang snatch",
    "hang power snatch",
    "snatch balance",
    "snatch pull",
    // Gymnastics
    "push up",
    "pull up",
    "chin up",
    "chest to bar",
    "muscle up",
    "bar muscle up",
    "ring muscleup",
    "toes to bar",
    "knees toelbows",
    "L-sit",
    "strict pull up",
    "handstand push up",
    "handstand walk",
    "handstand hold",
    // Other bar movements
    "thruster",
    "front rack lunge",
    "back rack lunge",
    "overhead walking lunge",
    // Other movements
    "burpee",
    "box jump",
    "burpee box jump",
    "burpee box jump over",
    "burpee over the bar",
    "burpee to target",
    "double under",
    "wall ball",
    "kettlebell swing",
    "turkish get up",
    "farmer's carry",
    "sled push",
    "sled pull",
    "sled drag",
    "sled sprint",
    // Cardio/machines
    "row",
    "run",
    "bike",
    "echo bike",
    "ski",
    // Dumbbell
    "dumbbell snatch",
    "dumbbell clean",
    "dumbbell power clean",
    "dumbbell hang clean",
    "devil press",
];

#[derive(Clone, Debug, PartialEq)]
pub enum Movement {
    AirSquat,
    FrontSquat,
    BackSquat,
    OverheadSquat,
    PistolSquat,
    GobletSquat,
    Deadlift,
    SumoDeadlift,
    RomanianDeadlift,
    ShoulderPress,
    PushPress,
    PushJerk,
    SplitJerk,
    BenchPress,
    Clean,
    PowerClean,
    HangClean,
    HangPowerClean,
    CleanAndJerk,
    PowerCleanAndJerk,
    CleanPull,
    Snatch,
    PowerSnatch,
    HangSnatch,
    HangPowerSnatch,
    SnatchBalance,
    SnatchPull,
    PushUp,
    PullUp,
    ChinUp,
    ChestToBar,
    MuscleUp,
    BarMuscleUp,
    RingMuscleUp,
    ToesToBar,
    KneesToElbows,
    LSit,
    StrictPullUp,
    HandstandPushUp,
    HandstandWalk,
    HandstandHold,
    Thruster,
    FrontRackLunge,
    BackRackLunge,
    OverheadWalkingLunge,
    Burpee,
    BoxJump,
    BurpeeBoxJump,
    BurpeeBoxJumpOver,
    BurpeeOverTheBar,
    BurpeeToTarget,
    DoubleUnder,
    WallBall,
    KettlebellSwing,
    TurkishGetUp,
    FarmersCarry,
    SledPush,
    SledPull,
    SledDrag,
    SledSprint,
    Row,
    Run,
    Bike,
    EchoBike,
    Ski,
    DumbbellSnatch,
    DumbbellClean,
    DumbbellPowerClean,
    DumbbellHangClean,
    DevilPress,
}

#[derive(Debug)]
pub enum MovementParseError {
    InvalidMovement(String),
}

impl fmt::Display for MovementParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MovementParseError::InvalidMovement(movement_name) => write!(
                f,
                "Invalid movement: `{}`, did you mean: `{}`?",
                movement_name,
                suggest_closest_movement(&movement_name).unwrap_or("None")
            ),
        }
    }
}

// Implement the std::error::Error trait for the custom error type
impl std::error::Error for MovementParseError {}

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "air squat" => Ok(Movement::AirSquat),
            "front squat" => Ok(Movement::FrontSquat),
            "back squat" => Ok(Movement::BackSquat),
            "overhead squat" => Ok(Movement::OverheadSquat),
            "pistol squat" => Ok(Movement::PistolSquat),
            "goblet squat" => Ok(Movement::GobletSquat),
            "deadlift" => Ok(Movement::Deadlift),
            "sumo deadlift" => Ok(Movement::SumoDeadlift),
            "romanian deadlift" => Ok(Movement::RomanianDeadlift),
            "shoulder press" => Ok(Movement::ShoulderPress),
            "push press" => Ok(Movement::PushPress),
            "push jerk" => Ok(Movement::PushJerk),
            "split jerk" => Ok(Movement::SplitJerk),
            "bench press" => Ok(Movement::BenchPress),
            "clean" => Ok(Movement::Clean),
            "power clean" => Ok(Movement::PowerClean),
            "hang clean" => Ok(Movement::HangClean),
            "hang power clean" => Ok(Movement::HangPowerClean),
            "clean and jerk" => Ok(Movement::CleanAndJerk),
            "power clean and jerk" => Ok(Movement::PowerCleanAndJerk),
            "clean pull" => Ok(Movement::CleanPull),
            "snatch" => Ok(Movement::Snatch),
            "power snatch" => Ok(Movement::PowerSnatch),
            "hang snatch" => Ok(Movement::HangSnatch),
            "hang power snatch" => Ok(Movement::HangPowerSnatch),
            "snatch balance" => Ok(Movement::SnatchBalance),
            "snatch pull" => Ok(Movement::SnatchPull),
            "push up" => Ok(Movement::PushUp),
            "pull up" => Ok(Movement::PullUp),
            "chin up" => Ok(Movement::ChinUp),
            "chest to bar" => Ok(Movement::ChestToBar),
            "muscle up" => Ok(Movement::MuscleUp),
            "bar muscle up" => Ok(Movement::BarMuscleUp),
            "ring muscleup" => Ok(Movement::RingMuscleUp),
            "toes to bar" => Ok(Movement::ToesToBar),
            "knees to elbows" => Ok(Movement::KneesToElbows),
            "L-sit" => Ok(Movement::LSit),
            "strict pull up" => Ok(Movement::StrictPullUp),
            "handstand push up" => Ok(Movement::HandstandPushUp),
            "handstand walk" => Ok(Movement::HandstandWalk),
            "handstand hold" => Ok(Movement::HandstandHold),
            "thruster" => Ok(Movement::Thruster),
            "front rack lunge" => Ok(Movement::FrontRackLunge),
            "back rack lunge" => Ok(Movement::BackRackLunge),
            "overhead walking lunge" => Ok(Movement::OverheadWalkingLunge),
            "burpee" => Ok(Movement::Burpee),
            "box jump" => Ok(Movement::BoxJump),
            "burpee box jump" => Ok(Movement::BurpeeBoxJump),
            "burpee box jump over" => Ok(Movement::BurpeeBoxJumpOver),
            "burpee over the bar" => Ok(Movement::BurpeeOverTheBar),
            "burpee to target" => Ok(Movement::BurpeeToTarget),
            "double under" => Ok(Movement::DoubleUnder),
            "wall ball" => Ok(Movement::WallBall),
            "kettlebell swing" => Ok(Movement::KettlebellSwing),
            "turkish get up" => Ok(Movement::TurkishGetUp),
            "farmer's carry" => Ok(Movement::FarmersCarry),
            "sled push" => Ok(Movement::SledPush),
            "sled pull" => Ok(Movement::SledPull),
            "sled drag" => Ok(Movement::SledDrag),
            "sled sprint" => Ok(Movement::SledSprint),
            "row" => Ok(Movement::Row),
            "run" => Ok(Movement::Run),
            "bike" => Ok(Movement::Bike),
            "echo bike" => Ok(Movement::EchoBike),
            "ski" => Ok(Movement::Ski),
            "dumbbell snatch" => Ok(Movement::DumbbellSnatch),
            "dumbbell clean" => Ok(Movement::DumbbellClean),
            "dumbbell power clean" => Ok(Movement::DumbbellPowerClean),
            "dumbbell hang clean" => Ok(Movement::DumbbellHangClean),
            "devil press" => Ok(Movement::DevilPress),
            _ => Err(MovementParseError::InvalidMovement(s.to_string())),
        }
    }
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Movement::AirSquat => "Air Squat",
                Movement::FrontSquat => "Front Squat",
                Movement::BackSquat => "Back Squat",
                Movement::OverheadSquat => "Overhead Squat",
                Movement::PistolSquat => "Pistol Squat",
                Movement::GobletSquat => "Goblet Squat",
                Movement::Deadlift => "Deadlift",
                Movement::SumoDeadlift => "Sumo Deadlift",
                Movement::RomanianDeadlift => "Romanian Deadlift",
                Movement::ShoulderPress => "Shoulder Press",
                Movement::PushPress => "Push Press",
                Movement::PushJerk => "Push Jerk",
                Movement::SplitJerk => "Split Jerk",
                Movement::BenchPress => "Bench Press",
                Movement::Clean => "Clean",
                Movement::PowerClean => "Power Clean",
                Movement::HangClean => "Hang Clean",
                Movement::HangPowerClean => "Hang Power Clean",
                Movement::CleanAndJerk => "Clean And Jerk",
                Movement::PowerCleanAndJerk => "Power Clean And Jerk",
                Movement::CleanPull => "Clean Pull",
                Movement::Snatch => "Snatch",
                Movement::PowerSnatch => "Power Snatch",
                Movement::HangSnatch => "Hang Snatch",
                Movement::HangPowerSnatch => "Hang Power Snatch",
                Movement::SnatchBalance => "Snatch Balance",
                Movement::SnatchPull => "Snatch Pull",
                Movement::PushUp => "Push Up",
                Movement::PullUp => "Pull Up",
                Movement::ChinUp => "Chin Up",
                Movement::ChestToBar => "Chest To Bar",
                Movement::MuscleUp => "Muscle Up",
                Movement::BarMuscleUp => "Bar Muscle Up",
                Movement::RingMuscleUp => "Ring Muscle Up",
                Movement::ToesToBar => "Toes To Bar",
                Movement::KneesToElbows => "Knees To Elbows",
                Movement::LSit => "L Sit",
                Movement::StrictPullUp => "Strict Pull Up",
                Movement::HandstandPushUp => "Handstand Push Up",
                Movement::HandstandWalk => "Handstand Walk",
                Movement::HandstandHold => "Handstand Hold",
                Movement::Thruster => "Thruster",
                Movement::FrontRackLunge => "Front Rack Lunge",
                Movement::BackRackLunge => "Back Rack Lunge",
                Movement::OverheadWalkingLunge => "Overhead Walking Lunge",
                Movement::Burpee => "Burpee",
                Movement::BoxJump => "Box Jump",
                Movement::BurpeeBoxJump => "Burpee Box Jump",
                Movement::BurpeeBoxJumpOver => "Burpee Box Jump Over",
                Movement::BurpeeOverTheBar => "Burpee Over The Bar",
                Movement::BurpeeToTarget => "Burpee To Target",
                Movement::DoubleUnder => "Double Under",
                Movement::WallBall => "Wall Ball",
                Movement::KettlebellSwing => "Kettlebell Swing",
                Movement::TurkishGetUp => "Turkish Get Up",
                Movement::FarmersCarry => "Farmers Carry",
                Movement::SledPush => "Sled Push",
                Movement::SledPull => "Sled Pull",
                Movement::SledDrag => "Sled Drag",
                Movement::SledSprint => "Sled Sprint",
                Movement::Row => "Row",
                Movement::Run => "Run",
                Movement::Bike => "Bike",
                Movement::EchoBike => "Echo Bike",
                Movement::Ski => "Ski",
                Movement::DumbbellSnatch => "Dumbbell Snatch",
                Movement::DumbbellClean => "Dumbbell Clean",
                Movement::DumbbellPowerClean => "Dumbbell Power Clean",
                Movement::DumbbellHangClean => "Dumbbell Hang Clean",
                Movement::DevilPress => "Devil Press",
            }
        )
    }
}

fn suggest_closest_movement(movement: &str) -> Option<&str> {
    // TODO: This will always return a str, change the output type
    // to just assume a string will be returned.
    let mut closest = None;
    let mut min_distance = usize::MAX;
    for m in MOVEMENTS {
        let distance = levenshtein(movement, m);
        if distance < min_distance {
            min_distance = distance;
            closest = Some(m.as_ref());
        }
    }
    closest
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_from_str_valid() {
        assert_eq!(Movement::from_str("air squat").unwrap(), Movement::AirSquat);
        assert_eq!(
            Movement::from_str("front squat").unwrap(),
            Movement::FrontSquat
        );
        assert_eq!(
            Movement::from_str("back squat").unwrap(),
            Movement::BackSquat
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(Movement::from_str("invalid movement").is_err());
        assert!(Movement::from_str("squat").is_err());
        assert!(Movement::from_str("air squa").is_err());
    }

    #[test]
    fn test_suggest_closest_movement() {
        assert_eq!(suggest_closest_movement("air squa"), Some("air squat"));
        assert_eq!(suggest_closest_movement("front s"), Some("front squat"));
        assert_eq!(suggest_closest_movement("back squ"), Some("back squat"));
        assert_eq!(suggest_closest_movement("snacth"), Some("snatch"));
    }

    #[test]
    fn test_error_message() {
        let err = Movement::from_str("clone").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid movement: `clone`, did you mean: `clean`?"
        );

        let err = Movement::from_str("squat").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid movement: `squat`, did you mean: `air squat`?"
        );
    }

    #[test]
    fn test_movement_display() {
        assert_eq!(format!("{}", Movement::AirSquat), "Air Squat");
        assert_eq!(format!("{}", Movement::FrontSquat), "Front Squat");
        assert_eq!(format!("{}", Movement::BackSquat), "Back Squat");
    }
}
