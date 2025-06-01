use std::fmt;
// For reference: https://www.crossfit.com/crossfit-movements
use std::collections::BTreeMap;
use std::str::FromStr;

use strsim::levenshtein;

/// Available movements
static MOVEMENTS: &[&str] = &[
    // Squat movements
    "air squat",
    "front squat",
    "back squat",
    "ohs",
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
    "clean-deadlift",
    "snatch",
    "power snatch",
    "hang snatch",
    "hang power snatch",
    "snatch balance",
    "snatch pull",
    "snatch deadlift",
    "muscle snatch",
    // Gymnastics
    "push up",
    "pull up",
    "chin up",
    "c2b",
    "chest to bar",
    "muscle up",
    "bar muscle up",
    "bar mu",
    "ring muscle up",
    "ring mu",
    "t2b",
    "toes to bar",
    "knees to elbows",
    "L-sit",
    "strict pull up",
    "shspu",
    "hspu",
    "handstand push up",
    "wall walk",
    "handstand walk",
    "hsw",
    "hs walk",
    "handstand hold",
    "sit up",
    "v up",
    "ghd",
    // Other bar movements
    "thruster",
    "front rack lunge",
    "back rack lunge",
    "overhead walking lunge",
    // Other movements
    "burpee",
    "box jump",
    "box jump over",
    "burpee box jump",
    "burpee box jump over",
    "burpee over the bar",
    "burpee to target",
    "du",
    "double under",
    "wall ball",
    "kettlebell swing",
    "kts",
    "turkish get up",
    "db farmer carry",
    "farmer carry",
    "sled push",
    "sled pull",
    "sled drag",
    "rope climb",
    "rc",
    "legless rope climb",
    "legless rc",
    "sandbag clean",
    "dball",
    "dball hold",
    "dball carry",
    // Cardio/machines
    "row",
    "run",
    "bike",
    "echo bike",
    "ski",
    // Dumbbell
    "db snatch",
    "db clean",
    "db power clean",
    "db hang clean",
    "dumbbell snatch",
    "db snatch",
    "dumbbell clean",
    "db clean",
    "dumbbell power clean",
    "db power clean",
    "dumbbell hang clean",
    "db hang clean",
    "dumbbell clean and jerk",
    "db clean and jerk",
    "db hang clean and jerk",
    "devil press",
];

/// Represents various types of movements that can be performed in a workout.
///
/// This enum includes a wide range of exercises from weightlifting and bodyweight training
/// to cardio and Olympic lifting. Each variant corresponds to a specific movement.
///
/// # Examples
///
/// ```
/// use wod::movement::Movement;
///
/// let movement = Movement::AirSquat;
/// println!("Movement: {}", movement);
/// ```
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
    CleanDeadlift,
    Snatch,
    PowerSnatch,
    HangSnatch,
    HangPowerSnatch,
    SnatchBalance,
    SnatchPull,
    SnatchDeadlift,
    MuscleSnatch,
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
    SitUp,
    VUp,
    GHD,
    StrictPullUp,
    StrictHandstandPushUp,
    HandstandPushUp,
    WallWalk,
    HandstandWalk,
    HandstandHold,
    Thruster,
    FrontRackLunge,
    BackRackLunge,
    OverheadWalkingLunge,
    Burpee,
    BoxJump,
    BoxJumpOver,
    BurpeeBoxJump,
    BurpeeBoxJumpOver,
    BurpeeOverTheBar,
    BurpeeToTarget,
    BurpeePullUp,
    DoubleUnder,
    WallBall,
    KettlebellSwing,
    TurkishGetUp,
    FarmersCarry,
    SledPush,
    SledPull,
    SledDrag,
    RopeClimb,
    LeglessRopeClimb,
    SandbagClean,
    DBall,
    DBallCarry,
    DBallHold,
    Row,
    Run,
    Bike,
    EchoBike,
    Ski,
    DumbbellSnatch,
    DumbbellClean,
    DumbbellPowerClean,
    DumbbellHangClean,
    DumbbellCleanAndJerk,
    DumbbellHangCleanAndJerk,
    DevilPress,
}

#[derive(Debug)]
pub enum MovementParseError {
    InvalidMovement(String, String),
}

impl fmt::Display for MovementParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MovementParseError::InvalidMovement(movement_name, suggestion) => write!(
                f,
                "Invalid movement: `{}`, did you mean: `{}`?",
                movement_name,
                suggestion // suggest_closest_movement(movement_name).unwrap_or("None")
            ),
        }
    }
}

// Implement the std::error::Error trait for the custom error type
impl std::error::Error for MovementParseError {}

impl MovementParseError {
    // Factory method that automatically suggests the closest movement.
    pub fn new_invalid(movement_name: String) -> Self {
        let suggestion = suggest_closest_movement(&movement_name);
        MovementParseError::InvalidMovement(movement_name, suggestion.unwrap().to_string())
    }
}

impl FromStr for Movement {
    type Err = MovementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "air squat" => Ok(Movement::AirSquat),
            "front squat" => Ok(Movement::FrontSquat),
            "back squat" => Ok(Movement::BackSquat),
            "ohs" => Ok(Movement::OverheadSquat),
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
            "clean-deadlift" => Ok(Movement::CleanDeadlift),
            "snatch" => Ok(Movement::Snatch),
            "power snatch" => Ok(Movement::PowerSnatch),
            "hang snatch" => Ok(Movement::HangSnatch),
            "hang power snatch" => Ok(Movement::HangPowerSnatch),
            "snatch balance" => Ok(Movement::SnatchBalance),
            "snatch pull" => Ok(Movement::SnatchPull),
            "snatch deadlift" => Ok(Movement::SnatchDeadlift),
            "muscle snatch" => Ok(Movement::MuscleSnatch),
            "push up" => Ok(Movement::PushUp),
            "pull up" => Ok(Movement::PullUp),
            "chin up" => Ok(Movement::ChinUp),
            "c2b" => Ok(Movement::ChestToBar),
            "chest to bar" => Ok(Movement::ChestToBar),
            "muscle up" => Ok(Movement::MuscleUp),
            "bar muscle up" => Ok(Movement::BarMuscleUp),
            "bar mu" => Ok(Movement::BarMuscleUp),
            "ring muscle up" => Ok(Movement::RingMuscleUp),
            "ring mu" => Ok(Movement::RingMuscleUp),
            "t2b" => Ok(Movement::ToesToBar),
            "toes to bar" => Ok(Movement::ToesToBar),
            "knees to elbows" => Ok(Movement::KneesToElbows),
            "L-sit" => Ok(Movement::LSit),
            "sit up" => Ok(Movement::SitUp),
            "v up" => Ok(Movement::VUp),
            "ghd" => Ok(Movement::GHD),
            "strict pull up" => Ok(Movement::StrictPullUp),
            "shspu" => Ok(Movement::StrictHandstandPushUp),
            "hspu" => Ok(Movement::HandstandPushUp),
            "handstand push up" => Ok(Movement::HandstandPushUp),
            "handstand walk" => Ok(Movement::HandstandWalk),
            "hs walk" => Ok(Movement::HandstandWalk),
            "hsw" => Ok(Movement::HandstandWalk),
            "wall walk" => Ok(Movement::WallWalk),
            "handstand hold" => Ok(Movement::HandstandHold),
            "thruster" => Ok(Movement::Thruster),
            "front rack lunge" => Ok(Movement::FrontRackLunge),
            "back rack lunge" => Ok(Movement::BackRackLunge),
            "overhead walking lunge" => Ok(Movement::OverheadWalkingLunge),
            "burpee" => Ok(Movement::Burpee),
            "box jump" => Ok(Movement::BoxJump),
            "box jump over" => Ok(Movement::BoxJumpOver),
            "burpee box jump" => Ok(Movement::BurpeeBoxJump),
            "burpee box jump over" => Ok(Movement::BurpeeBoxJumpOver),
            "burpee over the bar" => Ok(Movement::BurpeeOverTheBar),
            "burpee to target" => Ok(Movement::BurpeeToTarget),
            "burpee pull up" => Ok(Movement::BurpeePullUp),
            "du" => Ok(Movement::DoubleUnder),
            "double under" => Ok(Movement::DoubleUnder),
            "wall ball" => Ok(Movement::WallBall),
            "kettlebell swing" => Ok(Movement::KettlebellSwing),
            "turkish get up" => Ok(Movement::TurkishGetUp),
            "farmer carry" => Ok(Movement::FarmersCarry),
            "sled push" => Ok(Movement::SledPush),
            "sled pull" => Ok(Movement::SledPull),
            "sled drag" => Ok(Movement::SledDrag),
            "rope climb" => Ok(Movement::RopeClimb),
            "rc" => Ok(Movement::RopeClimb),
            "legless rope climb" => Ok(Movement::LeglessRopeClimb),
            "legless rc" => Ok(Movement::LeglessRopeClimb),
            "sandbag clean" => Ok(Movement::SandbagClean),
            "dball" => Ok(Movement::DBall),
            "dball carry" => Ok(Movement::DBallCarry),
            "dball hold" => Ok(Movement::DBallHold),
            "row" => Ok(Movement::Row),
            "run" => Ok(Movement::Run),
            "bike" => Ok(Movement::Bike),
            "echo bike" => Ok(Movement::EchoBike),
            "ski" => Ok(Movement::Ski),
            "db snatch" => Ok(Movement::DumbbellSnatch),
            "db clean" => Ok(Movement::DumbbellClean),
            "db power clean" => Ok(Movement::DumbbellPowerClean),
            "db hang clean" => Ok(Movement::DumbbellHangClean),
            "dumbbell snatch" => Ok(Movement::DumbbellSnatch),
            "dumbbell clean" => Ok(Movement::DumbbellClean),
            "dumbbell power clean" => Ok(Movement::DumbbellPowerClean),
            "dumbbell hang clean" => Ok(Movement::DumbbellHangClean),
            "dumbbell clean and jerk" => Ok(Movement::DumbbellCleanAndJerk),
            "db clean and jerk" => Ok(Movement::DumbbellCleanAndJerk),
            "devil press" => Ok(Movement::DevilPress),
            _ => Err(MovementParseError::new_invalid(s.to_string())),
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
                Movement::CleanDeadlift => "Clean Deadlift",
                Movement::Snatch => "Snatch",
                Movement::PowerSnatch => "Power Snatch",
                Movement::HangSnatch => "Hang Snatch",
                Movement::HangPowerSnatch => "Hang Power Snatch",
                Movement::SnatchBalance => "Snatch Balance",
                Movement::SnatchPull => "Snatch Pull",
                Movement::SnatchDeadlift => "Snatch Deadlift",
                Movement::MuscleSnatch => "Muscle Snatch",
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
                Movement::SitUp => "Sit Up",
                Movement::VUp => "V Up",
                Movement::GHD => "GHD",
                Movement::StrictPullUp => "Strict Pull Up",
                Movement::StrictHandstandPushUp => "Strict Handstand Push Up",
                Movement::HandstandPushUp => "Handstand Push Up",
                Movement::HandstandWalk => "Handstand Walk",
                Movement::WallWalk => "Wall Walk",
                Movement::HandstandHold => "Handstand Hold",
                Movement::Thruster => "Thruster",
                Movement::FrontRackLunge => "Front Rack Lunge",
                Movement::BackRackLunge => "Back Rack Lunge",
                Movement::OverheadWalkingLunge => "Overhead Walking Lunge",
                Movement::Burpee => "Burpee",
                Movement::BoxJump => "Box Jump",
                Movement::BoxJumpOver => "Box Jump Over",
                Movement::BurpeeBoxJump => "Burpee Box Jump",
                Movement::BurpeeBoxJumpOver => "Burpee Box Jump Over",
                Movement::BurpeeOverTheBar => "Burpee Over The Bar",
                Movement::BurpeeToTarget => "Burpee To Target",
                Movement::BurpeePullUp => "Burpee Pull Up",
                Movement::DoubleUnder => "Double Under",
                Movement::WallBall => "Wall Ball",
                Movement::KettlebellSwing => "Kettlebell Swing",
                Movement::TurkishGetUp => "Turkish Get Up",
                Movement::FarmersCarry => "Farmer's Carry",
                Movement::SledPush => "Sled Push",
                Movement::SledPull => "Sled Pull",
                Movement::SledDrag => "Sled Drag",
                Movement::RopeClimb => "Rope Climb",
                Movement::LeglessRopeClimb => "Legless Rope Climb",
                Movement::SandbagClean => "Sandbag Clean",
                Movement::DBall => "DBall",
                Movement::DBallCarry => "DBall Carry",
                Movement::DBallHold => "DBall Hold",
                Movement::Row => "Row",
                Movement::Run => "Run",
                Movement::Bike => "Bike",
                Movement::EchoBike => "Echo Bike",
                Movement::Ski => "Ski",
                Movement::DumbbellSnatch => "Dumbbell Snatch",
                Movement::DumbbellClean => "Dumbbell Clean",
                Movement::DumbbellPowerClean => "Dumbbell Power Clean",
                Movement::DumbbellHangClean => "Dumbbell Hang Clean",
                Movement::DumbbellCleanAndJerk => "Dumbbell Clean and Jerk",
                Movement::DumbbellHangCleanAndJerk => "Dumbbell Hang Clean and Jerk",
                Movement::DevilPress => "Devil Press",
            }
        )
    }
}

impl Movement {
    pub fn list_with_url() -> BTreeMap<String, String> {
        BTreeMap::from([
            (
                "Air Squat".to_string(),
                "https://www.crossfit.com/essentials/the-air-squat".to_string(),
            ),
            (
                "Front Squat".to_string(),
                "https://www.crossfit.com/essentials/the-front-squat".to_string(),
            ),
            (
                "Back Squat".to_string(),
                "https://www.crossfit.com/essentials/the-back-squat".to_string(),
            ),
            (
                "Overhead Squat".to_string(),
                "https://www.crossfit.com/essentials/the-overhead-squat".to_string(),
            ),
            (
                "Pistol Squat".to_string(),
                "https://www.crossfit.com/essentials/the-single-leg-squat".to_string(),
            ),
            ("Goblet Squat".to_string(), "".to_string()),
            (
                "Deadlift".to_string(),
                "https://www.crossfit.com/essentials/the-deadlift".to_string(),
            ),
            (
                "Sumo Deadlift".to_string(),
                "https://www.crossfit.com/essentials/the-sumo-deadlift".to_string(),
            ),
            ("Romanian Deadlift".to_string(), "".to_string()),
            (
                "Shoulder Press".to_string(),
                "https://www.crossfit.com/essentials/the-shoulder-press".to_string(),
            ),
            (
                "Push Press".to_string(),
                "https://www.crossfit.com/essentials/the-push-press".to_string(),
            ),
            (
                "Push Jerk".to_string(),
                "https://www.crossfit.com/essentials/the-push-jerk".to_string(),
            ),
            (
                "Split Jerk".to_string(),
                "https://www.crossfit.com/essentials/the-split-jerk".to_string(),
            ),
            (
                "Bench Press".to_string(),
                "https://www.crossfit.com/essentials/the-bench-press".to_string(),
            ),
            (
                "Clean".to_string(),
                "https://www.crossfit.com/essentials/the-clean-2".to_string(),
            ),
            (
                "Power Clean".to_string(),
                "https://www.crossfit.com/essentials/the-power-clean".to_string(),
            ),
            (
                "Hang Clean".to_string(),
                "https://www.crossfit.com/essentials/the-hang-squat-clean".to_string(),
            ),
            (
                "Hang Power Clean".to_string(),
                "https://www.crossfit.com/essentials/the-hang-power-clean".to_string(),
            ),
            (
                "Clean And Jerk".to_string(),
                "https://www.crossfit.com/essentials/the-clean-and-jerk".to_string(),
            ),
            (
                "Power Clean And Jerk".to_string(),
                "https://www.crossfit.com/essentials/the-squat-clean-and-push-jerk".to_string(),
            ),
            ("Clean Pull".to_string(), "".to_string()),
            ("Clean Deadlift".to_string(), "".to_string()),
            (
                "Snatch".to_string(),
                "https://www.crossfit.com/essentials/the-snatch".to_string(),
            ),
            (
                "Power Snatch".to_string(),
                "https://www.crossfit.com/essentials/the-power-snatch".to_string(),
            ),
            (
                "Hang Snatch".to_string(),
                "https://www.crossfit.com/essentials/the-hang-snatch".to_string(),
            ),
            (
                "Hang Power Snatch".to_string(),
                "https://www.crossfit.com/essentials/the-hang-power-snatch".to_string(),
            ),
            (
                "Snatch Balance".to_string(),
                "https://www.crossfit.com/essentials/the-snatch-balance".to_string(),
            ),
            ("Snatch Pull".to_string(), "".to_string()),
            ("Snatch Deadlift".to_string(), "".to_string()),
            (
                "Muscle Snatch".to_string(),
                "https://www.crossfit.com/essentials/the-muscle-snatch".to_string(),
            ),
            ("Push Up".to_string(), "".to_string()),
            ("Pull Up".to_string(), "".to_string()),
            ("Chin Up".to_string(), "".to_string()),
            ("Chest To Bar".to_string(), "".to_string()),
            ("Muscle Up".to_string(), "".to_string()),
            ("Bar Muscle Up".to_string(), "".to_string()),
            ("Ring Muscle Up".to_string(), "".to_string()),
            ("Knees To Elbows".to_string(), "".to_string()),
            ("L Sit".to_string(), "".to_string()),
            ("Strict Pull Up".to_string(), "".to_string()),
            ("Strict Handstand Push Up".to_string(), "".to_string()),
            ("Handstand Push Up".to_string(), "".to_string()),
            ("Handstand Walk".to_string(), "".to_string()),
            ("Wall Walk".to_string(), "".to_string()),
            ("Handstand Hold".to_string(), "".to_string()),
            ("Front Rack Lunge".to_string(), "".to_string()),
            ("Back Rack Lunge".to_string(), "".to_string()),
            ("Overhead Walking Lunge".to_string(), "".to_string()),
            ("Burpee".to_string(), "".to_string()),
            ("Box Jump".to_string(), "".to_string()),
            ("Box Jump Over".to_string(), "".to_string()),
            ("Burpee Box Jump".to_string(), "".to_string()),
            ("Burpee Box Jump Over".to_string(), "".to_string()),
            ("Burpee Over The Bar".to_string(), "".to_string()),
            ("Burpee To Target".to_string(), "".to_string()),
            ("Double Under".to_string(), "".to_string()),
            ("Rope Climb".to_string(), "".to_string()),
            ("Wall Ball".to_string(), "".to_string()),
            ("Kettlebell Swing".to_string(), "".to_string()),
            ("Turkish Get Up".to_string(), "".to_string()),
            ("Farmer's Carry".to_string(), "".to_string()),
            ("Sandbag Clean".to_string(), "".to_string()),
            ("Sled Push".to_string(), "".to_string()),
            ("Sled Pull".to_string(), "".to_string()),
            ("Sled Drag".to_string(), "".to_string()),
            ("Row".to_string(), "".to_string()),
            ("Run".to_string(), "".to_string()),
            ("Bike".to_string(), "".to_string()),
            ("Echo Bike".to_string(), "".to_string()),
            ("Ski".to_string(), "".to_string()),
            ("Dumbbell Snatch".to_string(), "".to_string()),
            ("Dumbbell Clean".to_string(), "".to_string()),
            ("Dumbbell Power Clean".to_string(), "".to_string()),
            ("Dumbbell Hang Clean".to_string(), "".to_string()),
            ("Dumbbell Clean And Jerk".to_string(), "".to_string()),
            ("Dumbbell Hang Clean And Jerk".to_string(), "".to_string()),
            ("Devil Press".to_string(), "".to_string()),
        ])
    }
}

fn suggest_closest_movement(movement: &str) -> Option<&'static str> {
    // TODO: This will always return a str, change the output type
    // to just assume a string will be returned.
    let mut closest = None;
    let mut min_distance = usize::MAX;
    for &m in MOVEMENTS {
        let distance = levenshtein(movement, m);
        if distance < min_distance {
            min_distance = distance;
            closest = Some(m);
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
