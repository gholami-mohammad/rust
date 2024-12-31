fn main() {
    let p = Person {
        first_name: "Mohammad".to_string(), // converting &str to String
        last_name: String::from("Gholami"),
        age: 35,
        is_student: false,
        years_of_experience: 10,
    };

    println!("person is: {:?}", p);

    // p.first_name = "Kaveh".to_string(); // this action is not allowed because p in immutable

    let mut p2 = Person {
        first_name: String::from("Ali"),
        last_name: String::from("Andalibi"),
        age: 37,
        is_student: false,
        years_of_experience: 15,
    };

    p2.age = 38; // p2 is mutable, so we can change values
    println!("person is: {:?}", p2);

    // moving struct field ownership
    let p2name = p2.first_name;
    println!("p2name is: {}", p2name);
    // println!("person 2 first_name: {}", p2.first_name); // compile error. p2.first_name ownership already moved to p2name

    let p2last_name = p2.last_name.clone(); // we can use clone to make copy instead on moving ownership
    println!(
        "p2last_name is: {} and p2.last_name is: {}",
        p2last_name, p2.last_name
    );

    // keep in mind: primitive data types always living in stack
    // other types live in heap
    // stack data always is copied and have no ownership
    // heap data has ownership and ownership is moved
    // for example:
    let p2_age = p2.age; // age is u8 => it is in stack => no ownership. Compare it to line 25-28
    println!("p2_age is: {}, p2.age is: {}", p2_age, p2.age);

    // partial move
    // we can copy or move some fields of a struct to another struct
    // for example:
    let p3 = Person {
        first_name: "Zari".to_string(),
        last_name: "Aghaei".to_string(),

        // this will set other fields of Person equal to the p
        // key point is again stack data will be copied
        // heap data ownership will be moved
        ..p
    };
    println!("p3 is: {:?}", p3);

    let p4 = Person { age: 45, ..p };
    println!("p4 is {:?}", p4);
    println!("p4.first_name: {}", p4.first_name);
    // println!("p.last_name: {}", p.last_name); // => compiler error: p.last_name moved to p4 and no longer valid

    // ============= Tuple Struct ===============
    // 3 ways of defining tuples:

    // 1
    let p_2d: (i32, i32) = (12, 45);
    println!("p_2d is: {:?}", p_2d);

    // 2: creating a type alias
    type P2D = (i32, i32);
    let p1_2d: P2D = (12, 55);
    println!("p1_2d: {:?}", p1_2d);

    // 3: tuple struct
    #[derive(Debug)]
    struct Point2D(i32, i32);
    let p3: Point2D = Point2D(44, 66);
    let p4 = Point2D(98, 34);
    println!("p3: {:?}, p4: {:?}", p3, p4);
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    is_student: bool,
    years_of_experience: u8,
}
