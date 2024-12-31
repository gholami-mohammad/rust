fn main() {
    // ============== FOR =================
    // iterating over a range of numbers
    // form 0 to 9
    println!("iterate over 0-9");
    for i in 0..10 {
        println!("{}", i);
    }
    println!("-----------------------");

    // zero to 10
    println!("iterate over 0-10");
    for i in 0..=10 {
        println!("{}", i);
    }
    println!("-----------------------");

    // iterating over vector
    let v: Vec<i32> = vec![5, 7, 4, 34, 234, 542, 342];
    println!("iterate over vector");
    for i in v {
        println!("{}", i);
    }
    println!("-----------------------");

    // iterating over array
    let arr: [&str; 5] = ["Mohammad", "Ali", "Mehrzad", "Reza", "Sina"];
    println!("iterate over array");
    for name in arr {
        println!("{}", name);
    }
    println!("-----------------------");

    // ============== WHILE =================
    let mut m = 0;
    while m < 5 {
        println!("while m var is: {}", m);
        m += 1;
    }
    // ============== LOOP =================
    let mut i = 0;
    loop {
        println!("{}", i);
        i += 1;
        if i == 10 {
            break; // to return from current loop
        }
    }

    let mut j = 0;
    let mut k = 0;
    // labeling loop
    'outer: loop {
        loop {
            println!("j: {}, k: {}", j, k);
            k += 1;
            if k == 4 {
                break 'outer; // to return to specified loop
            } else {
                break; // to return from current loop
            }
        }
        j += 1
    }

    // set loop break result to a variable
    let active = true;
    let x = loop {
        if active {
            break 10;
        } else {
            break 20;
        }
    };
    println!("x is {}", x);

    // infinitive loop
    println!("infinitive loop");
    loop {}
}
