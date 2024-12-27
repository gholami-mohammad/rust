struct Student {
    name: String,
    grade: Option<u32>, // Option is an enum with Some(T) or None. None is same as null
}

// student_name and db both are borrowed an immutable references to original data
// this function can not modify none of these arguments. also does not take the data ownership
fn get_student_grade_by_name(student_name: &String, db: &Vec<Student>) -> Option<u32> {
    for student in db {
        if student.name == *student_name {
            return student.grade;
        }
    }

    return None;
}

fn main() {
    let students_database: Vec<Student> = vec![
        Student {
            name: String::from("Mohammad"),
            grade: Some(90),
        },
        Student {
            name: String::from("Ali"),
            grade: None,
        },
        Student {
            name: "Kaveh".to_string(),
            grade: Some(100),
        },
    ];

    let m = String::from("Mohammad");
    let grade = get_student_grade_by_name(&m, &students_database);
    match grade {
        Some(g) => println!("{}'s grade is {}", m, g),
        _ => {} // omit other cases
    }
    // using if-let
    if let Some(g) = grade {
        println!("{}'s grade is {}", m, g);
    }

    let a = String::from("Ali");
    let grade2 = get_student_grade_by_name(&a, &students_database);
    println!("{}'s grade is {:?}", a, grade2.unwrap_or(0));
    // also using match
    match grade2 {
        Some(g) => println!("{}'s grade is {}", a, g),
        None => println!("{} has no grade", a),
    }

    // using if-let
    if let Some(g) = grade2 {
        println!("{}'s grade is {}", a, g);
    } else {
        println!("{} has no grade", a);
    }

    // using None in if-let
    if let None = grade2 {
        println!(
            "{} has no grade, we should notify the teacher to submit the grade",
            a
        );
    }
}
