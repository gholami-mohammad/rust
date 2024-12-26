fn main() {
    my_fist_function("Hello, world!");

    let x: &str = "this is a new string";
    my_fist_function(x);

    let a = multiplication(10, 20);
    println!("multiplication is: {}", a);

    println!("multiplication2 is: {}", multiplication2(10, 22));

    let (sum, diff, mul) = function_with_multiple_return_values(10, 5);
    println!("sum: {}, diff: {}, mul: {}", sum, diff, mul);

    // ignore some returned values
    let (sum, _, mul) = function_with_multiple_return_values(100, 33);
    println!("sum: {}, mul: {}", sum, mul);

    // Code block
    {
        let x = 10;
        println!("x value is in code block: {}", x);
    }

    // returning data from a code block
    let full_name = {
        let first_name = "Mohammd";
        let last_name = "Gholami";

        format!("{} {}", first_name, last_name) // no semicolon at the end of the statement
    };
    println!("full_name value is: {}", full_name);

    let s = {
        let a = 10;
        let b = 120;
        a + b // no semicolon at the end of the statement to return data from code block
    };
    println!("s value is: {}", s);
}

// naming convention is snake_case for function names
fn my_fist_function(s: &str) {
    println!("my_first_function s value is: {}", s);
}

// function with return value
// return type is specified after the -> arrow
fn multiplication(a: i32, b: i32) -> i32 {
    return a * b;
}

// returning data without using return keyword
fn multiplication2(a: i32, b: i32) -> i32 {
    // to return without using return keyword, remove the semicolon at the end of the statement
    a * b // equal to return a * b;
}

fn function_with_multiple_return_values(a: i32, b: i32) -> (i32, i32, i32) {
    let sum = a + b;
    let diff = a - b;
    let mul = a * b;

    return (sum, diff, mul);
}
