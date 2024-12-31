fn main() {
    // ================ IF-ELSE ================
    // simple if-else
    let a = 2;
    if a < 20 {
        println!("a is less than 20")
    } else {
        println!("a is greater than 20")
    }

    // assigning if else block expression to a variable
    let mark = 78;
    // in this case, all branches of if-else should return same datatype as expression
    let grade = if mark > 90 {
        'A' // like returning data in code block. no semicolon required
    } else if mark > 80 {
        'B'
    } else {
        'C'
    }; // this semicolon should be added in case of assigning if-else result to a variable

    println!("Grade is; {}", grade);

    // ================ MATCH ================
    let mark = 77;
    let g: char;
    match mark {
        // .. syntax is use to express rage of numbers
        // 90..100 means >= 90 and < 100
        // 90..=100 means >= 90 and <= 100
        90..=100 => g = 'A',
        80..=89 => g = 'B',
        70..=79 => g = 'C',
        60..=69 => g = 'D',
        _ => g = 'F', // default if nothing matched
    }
    println!("grade using match is: {g}");

    // match with multi line expression
    let skill = "Rust";
    let class: &str;
    match skill {
        "Rust" | "Golang" => {
            class = "Backend";
            println!("your skill is: {skill}");
        }
        "Angular" | "React" => {
            class = "Frontend";
            println!("your skill is: {skill}");
        }
        _ => class = "NA",
    }
    println!("class is: {class}");

    // assign match result to a variable
    let skill = "Golang";
    let class = match skill {
        "Rust" | "Golang" => {
            println!("your skill is: {skill}");
            "Backend" // no semicolon to be like an expression
        }
        "Angular" | "React" => "Frontend",
        _ => "NA",
    };
    println!("class is: {class}");
}
