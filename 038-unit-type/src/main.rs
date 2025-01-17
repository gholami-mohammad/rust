fn returning_nothing() {
    // do something without returning any value
}

fn validate_being_old_enough(x: i32) -> Result<(), String> {
    if x >= 18 {
        // here we are returning a unit value
        Ok(())
    } else {
        Err("you are not old enough".to_string())
    }
}

fn main() {
    // In Rust, the unit type is written as (). It represents the absence of any value or data

    // returning value of print! macro is a unit type
    let x = print!("print me");
    println!("Hello, world!");

    let y = returning_nothing();

    let is_old = validate_being_old_enough(32);
    match is_old {
        Ok(()) => println!("you are old enough"),
        Err(e) => println!("you are too young"),
    }

    // vectors with unit type
    // we initialized the vector with 0 capacity for now
    // and then add some data
    let mut v: Vec<()> = Vec::with_capacity(0);
    // we can push data on this vector
    // but there is no heap allocation due to its structure: all elements are unit type -> so no memory allocation
    v.push(());
    v.push(());
    v.push(());
    v.push(());
    assert_eq!(4, v.len());
}
