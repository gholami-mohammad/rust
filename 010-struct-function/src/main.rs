struct Car {
    owner: String,
    year: u8,
    fuel_level: f32,
    price: u32,
}

impl Car {
    // &self is immutable reference to Car struct
    fn display_car_info(&self) {
        println!(
            "car owner: {}; car year: {}, fuel_level: {}, price: {}",
            self.owner, self.year, self.fuel_level, self.price
        );
    }

    // &mut self is a mutable reference to car
    fn refuel(&mut self, liters: f32) {
        self.fuel_level += liters;
    }

    // self with small "s" is immutable reference to current instance
    // Self in return with capital S, is new instance of current instance by copying the data
    fn sell(mut self, new_owner_name: String) -> Self {
        self.owner = new_owner_name;
        self
    }

    // associated function are like static functions in PHP
    // to call this function we use this syntax: Car::the_associated_func();
    fn the_associated_func() {
        println!("This is an associated function of car");
    }

    // new function is used to create new instance of Car
    fn new(owner: String, year: u8) -> Self {
        let ins = Self {
            owner,
            year,
            fuel_level: 0.0,
            price: 0,
        };

        return ins;
    }
}

fn main() {
    let mgh_car = Car {
        owner: "Mohammd".to_string(),
        year: 2,
        fuel_level: 23.6,
        price: 548_000,
    };

    mgh_car.display_car_info();
    // mgh_car.refuel(20.0); // this will return compiler error because mgh_car is immutable
    // to solve look at this example:
    let mut z_car = Car {
        owner: "Zari".to_string(),
        year: 2,
        fuel_level: 23.6,
        price: 548_000,
    };

    z_car.display_car_info();
    z_car.refuel(20.0); // now we can mutate data
    z_car.display_car_info();

    let mut new_owner = z_car.sell("Leila".to_string()); // ownership moved here
    new_owner.display_car_info();
    // z_car.refuel(10.3); // compile error. z_car ownership already moved to new_owner
    new_owner.refuel(9.4);
    new_owner.display_car_info();

    Car::the_associated_func();

    let mut using_new = Car::new("Parand".to_string(), 1);
    using_new.refuel(23.4);
    using_new.display_car_info();
}
