struct Employee {
    name: String,
    salary: u32,
}

struct EmployeeRecords {
    employee_db: Vec<Employee>,
}

// we want to implement the iterator for EmployeeRecords to iterate over all employee names
// and remove them from vector list when name read.
impl Iterator for EmployeeRecords {
    // we can also define the item type to Employee.
    // this is an associated type and we can define it as we wish.
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employee_db.len() == 0 {
            return None;
        }

        // clone fist index
        let the_name = self.employee_db[0].name.clone();
        // remove the entry from list after reading it
        self.employee_db.remove(0);

        return Some(the_name);
    }
}

fn main() {
    let emp_1 = Employee {
        name: "Sam".to_string(),
        salary: 120_000,
    };
    let emp_2 = Employee {
        name: "Kaveh".to_string(),
        salary: 150_000,
    };
    let emp_3 = Employee {
        name: "John".to_string(),
        salary: 110_000,
    };
    let emp_4 = Employee {
        name: "Due".to_string(),
        salary: 100_000,
    };

    let mut db = EmployeeRecords {
        employee_db: vec![emp_1, emp_2, emp_3, emp_4],
    };

    println!("1st employee: {:?}", db.next());
    println!("2nd employee: {:?}", db.next());
    println!("3rd employee: {:?}", db.next());
    println!("4th employee: {:?}", db.next());
    println!("5th employee: {:?}", db.next());

    println!("=============== using for loop ================");

    let emp_1 = Employee {
        name: "Sam".to_string(),
        salary: 120_000,
    };
    let emp_2 = Employee {
        name: "Kaveh".to_string(),
        salary: 150_000,
    };
    let emp_3 = Employee {
        name: "John".to_string(),
        salary: 110_000,
    };
    let emp_4 = Employee {
        name: "Due".to_string(),
        salary: 100_000,
    };

    let db_2 = EmployeeRecords {
        employee_db: vec![emp_1, emp_2, emp_3, emp_4],
    };

    // print using for loop
    // if using for loop over a type which implemented Iterator,
    // Rust will automatically call the next function on it and read the result.
    for emp in db_2 {
        println!("user name is: {}", emp);
    }
}
