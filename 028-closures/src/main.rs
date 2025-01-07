struct User {
    name: String,
    age: u32,
    salary: u32,
}

fn is_valid_user<Validator1, Validator2>(user: &User, v1: Validator1, v2: Validator2) -> bool
where
    Validator1: Fn(&str) -> bool,
    Validator2: Fn(u32) -> bool,
{
    v1(&user.name) && v2(user.age)
}

fn main() {
    // 3 type of closures:
    // 1: function with immutable borrow like: validate_name and validate_age_over_30
    // 2: function with mutable reference like:should_update_salary_if_over_30
    // 3: function which moves ownership like: banned_user_checker

    let validate_name = |name: &str| name.len() > 0;
    let validate_age_over_30 = |age: u32| {
        return age >= 30;
    };

    let mut new_salary = 78_000;
    let mut should_update_salary_if_over_30 = |age: u32| {
        let ns = &mut new_salary;

        if age >= 30 {
            return true;
        }

        return false;
    };

    let banned_user_name = String::from("Banned User");

    let banned_user_checker = |name: &str| {
        let b = banned_user_name;

        name == b
    };

    let mut user_1 = User {
        name: "MGH".to_string(),
        age: 36,
        salary: 55_000,
    };

    println!("user_1 name validation: {}", validate_name(&user_1.name));
    println!(
        "user_1 age validation: {}",
        validate_age_over_30(user_1.age)
    );

    let is_valid = is_valid_user(&user_1, validate_name, validate_age_over_30);
    println!("user full validation check result: {}", is_valid);

    let should_update = should_update_salary_if_over_30(user_1.age);
    println!("should update salary: {}", should_update);

    let is_banned = banned_user_checker(&user_1.name);
    println!("user is banned: {}", is_banned);
}
