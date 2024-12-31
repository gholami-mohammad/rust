struct Student {
    name: String,
    grade: Option<u32>, // Option is an enum with Some(T) or None. None is same as null
}

// check student database. If student does not exists, returning an error
// otherwise returning its grade
fn get_student_grade_by_name(
    student_name: &String,
    db: &Vec<Student>,
) -> Result<Option<u32>, String> {
    for student in db {
        if student.name == *student_name {
            return Ok(student.grade);
        }
    }

    return Err(String::from("student not found"));
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

    let name = String::from("Kaveh");

    let result = get_student_grade_by_name(&name, &students_database);

    match result {
        Ok(grade) => match grade {
            Some(g) => println!("student {} has grade {}", name, g),
            None => println!("student {} has no grade", name),
        },
        Err(error) => {
            println!("failed to get student grade: {}", error);
        }
    }

    let name = String::from("No One");

    let result = get_student_grade_by_name(&name, &students_database);

    match result {
        Ok(grade) => match grade {
            Some(g) => println!("student {} has grade {}", name, g),
            None => println!("student {} has no grade", name),
        },
        Err(error) => {
            println!("failed to get student grade: {}", error);
        }
    }
}
