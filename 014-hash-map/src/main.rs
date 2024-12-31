use std::collections::HashMap;

fn main() {
    let mut person_ages: HashMap<&str, u8> = HashMap::new();

    person_ages.insert("Mohammad", 35);
    person_ages.insert("Kaveh", 35);
    person_ages.insert("Diba", 8);

    println!("{:?}", person_ages);

    let diba = person_ages.get("Diba");
    println!("{:?}", diba);

    // check key existence
    if person_ages.contains_key("Ali") {
        println!("Ali exists in the hash map");
    } else {
        println!("Ali does not exist in the hash map");
    }

    // use match expression to get value from hash map
    match person_ages.get("Zari") {
        Some(age) => println!("Zari is {} years old", age),
        None => println!("Zari is not in the hash map"),
    }

    // iterating over all key values of hash map
    // use &person_ages instead of person_ages to not moving ownership
    for (name, age) in &person_ages {
        println!("{} is {} years old", name, age);
    }

    // overwrite existing value
    person_ages.insert("Siavash", 20);
    person_ages.insert("Siavash", 30); // updates value of Siavash

    // insert a key if only it does not exists
    person_ages.entry("Jamshid").or_insert(43); // will insert Jamshid because it does not exists
    person_ages.entry("Kaveh").or_insert(43); // it wont update kaveh because it already exists. or_insert returns a reference to the value exist or inserted

    println!("Kaveh {:?}", person_ages.get("Kaveh")); // will print 35

    // an other example
    calculate_frequency();
}

fn calculate_frequency() {
    let ages: Vec<u8> = vec![12, 4, 23, 4, 5, 3, 3, 5, 8, 3, 21, 12, 19, 18];
    let mut freq: HashMap<u8, u32> = HashMap::new();

    for i in ages {
        let f: &mut u32 = freq.entry(i).or_insert(0);
        *f += 1; // updating value stored in reference of f
    }

    for (age, f) in freq {
        println!("age: {} repeated {} time(s)", age, f);
    }
}
