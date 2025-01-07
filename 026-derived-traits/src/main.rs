use std::fmt::Debug;

// marker trait is a trait that does not have any method to implement
// marker traits are used to add metadata other structs or traits
// we can add inherited traits to a marker trait.
// Now, any trait which implement Properties trait, should implement PartialEq + Clone + Debug
// look at User struct
trait Properties: PartialEq + Clone + Debug {}

#[derive(PartialEq, Clone, Debug)]
struct User {
    id: u32,
    code: String,
    name: String,
}
impl Properties for User {} // due to this implementation, we should add derive attribute for PartialEq + Clone + Debug to user struct

#[derive(Debug, PartialEq)] // we can use this syntax to derive a macro into our struct
struct Student {
    id: u32,
    name: String,
    active: bool,
}

fn main() {
    let s = Student {
        id: 1,
        name: String::from("MGH"),
        active: true,
    };

    println!("student: {:?}", s); // we need Debug

    let s1 = Student {
        id: 2,
        name: String::from("MGH"),
        active: true,
    };

    println!("check s is equal to s1: {}", s == s1) // we need PartialEq
}
