enum WeekDay {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

// we can add functions to enums
impl WeekDay {
    fn get_work_hour(&self) -> (i32, i32) {
        let (start, end) = match self {
            WeekDay::Saturday | WeekDay::Monday => (8, 16),
            WeekDay::Sunday | WeekDay::Tuesday => (12, 20),
            WeekDay::Wednesday | WeekDay::Thursday | WeekDay::Friday => (9, 17),
        };

        (start, end)
    }
}

// enum with input data
enum MonitorType {
    HD(f32), // f32 is the inch size of the monitor
    FHD(f32),
    QHD(f32),
    UHD(f32),
    OLED(f32),
    QLED(f32),
}

impl MonitorType {
    fn get_price(&self) -> f32 {
        match self {
            Self::HD(inch) => inch * 4.56,
            Self::FHD(inch) => inch * 5.22,
            Self::QHD(inch) => inch * 5.44,
            Self::UHD(inch) => inch * 6.0,
            Self::OLED(inch) => inch * 8.22,
            Self::QLED(inch) => inch * 10.0,
        }
    }
}

fn main() {
    let day = WeekDay::Friday;
    let wh = day.get_work_hour();
    println!("working hours for friday is: from {} to {}", wh.0, wh.1);

    let day2 = WeekDay::Sunday;
    let (s_from, s_to) = day2.get_work_hour();
    println!("working hours for sunday is: from {} to {}", s_from, s_to);

    let my_mponitor = MonitorType::HD(27.0);
    let price = my_mponitor.get_price();
    println!("price of my monitor is: {}", price);
}
