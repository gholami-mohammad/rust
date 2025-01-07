// In Rust, an associated type is a feature used within traits to define types that are specific to each implementation of the trait. Here's a brief explanation:
// Associated types are declared inside a trait with the type keyword, specifying a placeholder type that each implementer of the trait must define.

struct Kilometer {
    value: u32,
}

struct KilometerPerHour {
    value: u32,
}

struct Mile {
    value: u32,
}

struct MilePerHour {
    value: u32,
}

trait ThreeHourDistanceCalculator {
    type Distance; // this is associated type.

    fn get_distance_in_three_hours(&self) -> Self::Distance; // returning associated type as function return type
}

// now, lets implement ThreeHourDistanceCalculator for KilometerPerHour and MilePerHour
impl ThreeHourDistanceCalculator for KilometerPerHour {
    // this means, Distance in ThreeHourDistanceCalculator is associated with Kilometer
    // so get_distance_in_three_hours return type will be Kilometer
    type Distance = Kilometer;
    fn get_distance_in_three_hours(&self) -> Self::Distance {
        Kilometer {
            value: self.value * 3,
        }
    }
}

impl ThreeHourDistanceCalculator for MilePerHour {
    type Distance = Mile;

    fn get_distance_in_three_hours(&self) -> Self::Distance {
        return Mile {
            value: self.value * 3,
        };
    }
}

fn main() {
    let kmh = KilometerPerHour { value: 100 };
    println!(
        "kilometers traveled with speed of {} will be {} km",
        100,
        kmh.get_distance_in_three_hours().value,
    );

    let mh = MilePerHour { value: 60 };
    println!(
        "miles traveled with speed of {} will be {} miles",
        60,
        mh.get_distance_in_three_hours().value,
    );
}
