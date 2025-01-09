struct User {
    name: String,
    age: u8,
}
fn main() {
    // consider we have a list of users
    // we what to filter out only users with age above 35
    // then convert their name to upper case and save result into new array.
    // we can use combinator functions like this:

    let users: Vec<User> = vec![
        User {
            name: "Kaveh".to_string(),
            age: 35,
        },
        User {
            name: "Zari".to_string(),
            age: 31,
        },
        User {
            name: "Ali".to_string(),
            age: 37,
        },
        User {
            name: "Mohammad Reza".to_string(),
            age: 23,
        },
    ];

    let old_guys = users.
    into_iter().//this is a combinator
    filter(|user| user.age >= 35).// an other combinator
    map(|user| User{name: user.name.to_uppercase(), age: user.age}).
    collect::<Vec<User>>(); //  turbofish: SEE turbofish

    // turbofish:
    // In Rust, the syntax collect::<Vec<User>>() is known as turbofish syntax. It's used to explicitly specify the type parameter for a method or function, particularly when the compiler cannot infer the type automatically. Here, Vec<User> is the type being specified for the collect method, indicating that the result should be collected into a Vec (vector) of User structs.

    for g in old_guys {
        println!("name: {} age: {}", g.name, g.age);
    }
}
