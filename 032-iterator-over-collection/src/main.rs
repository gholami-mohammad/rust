use std::collections::HashMap;

fn main() {
    let user_ids = vec![10, 23, 123, 233, 342, 124, 553, 553, 334, 552, 556];

    // iterating over vector with immutable reference to values
    println!(" ---------- vector immutable reference ---------");
    for id in &user_ids {
        println!("{id}");
    }
    // user_ids still accessible here
    println!("ids 1: {:?}", user_ids);

    println!(" ---------- vector mutable reference ---------");
    // creating a mutable vector, then iterating over items using mutable reference
    let mut user_ids = vec![10, 23, 123, 233, 342, 124, 553, 553, 334, 552, 556];
    for id in &mut user_ids {
        println!("{id}");
    }

    println!(" ---------- vector moved ownership ---------");
    let user_ids = vec![10, 23, 123, 233, 342, 124, 553, 553, 334, 552, 556];
    for id in user_ids {
        println!("{id}");
    }
    // println!("ids 1: {:?}", user_ids); // -> this will be compiler error due ownership error

    // -------- Hash Map ----------
    let mut persons: HashMap<String, u32> = HashMap::new();
    persons.insert("Kaveh".to_string(), 35);
    persons.insert("Zari".to_string(), 31);
    persons.insert("Parand".to_string(), 36);

    // immutable reference
    println!(" ---------- hashmap using immutable reference ---------");
    for (name, age) in &persons {
        println!("{} is {} years old", name, age);
    }

    println!(" ---------- hashmap using mutable reference ---------");
    // only value of hashmap will be mutable. key will remain immutable
    for (name, age) in &mut persons {
        println!("{} is {} years old", name, age);
    }

    println!(" ---------- hashmap with moved ownership ---------");
    for (name, age) in persons {
        println!("{} is {} years old", name, age);
    }

    // println!("{:?}", persons); // persons will not be available here due to moved ownership
}
