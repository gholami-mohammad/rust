struct User {
    name: String,
    age: u32,
    salary: u32,
}

fn is_valid_user(
    user: &User,
    name_validator: fn(&str) -> bool, // compare it with closure in previous example
    age_validator: fn(u32) -> bool,
) -> bool {
    name_validator(&user.name) && age_validator(user.age)
}

fn age_validator(age: u32) -> bool {
    age >= 30
}

fn is_valid_user_2<V1, V2>(user: &User, v1: V1, v2: V2) -> bool
where
    V1: Fn(&str) -> bool,
    V2: Fn(u32) -> bool,
{
    v1(&user.name) && v2(user.age)
}

fn main() {
    let user_1 = User {
        name: String::from("MGH"),
        age: 36,
        salary: 90_000,
    };

    let name_validator = |name: &str| name.len() > 0;

    // i'm passing name_validator (closure) instead of function pointer
    let is_valid = is_valid_user(&user_1, name_validator, age_validator);
    println!("user is valid: {}", is_valid);

    // here, i used function reference instead of closure
    let is_valid_2 = is_valid_user_2(&user_1, name_validator, age_validator);
    println!("user is valid 2: {}", is_valid_2);
}
