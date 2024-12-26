fn main() {
    println!("Hello, world!");

    print!("Printing on the same line ");
    print!("continue printing on the same line\n");

    println!("Printing tab space: \t after tab");

    // The \r carriage return character in Rust, and many other languages, doesn't overwrite the entire line's content, it resets the cursor to the beginning of the line. Subsequent output then overwrites characters from the start of the line. If the new output is shorter than the existing line, the remaining characters from the original line will still be visible
    print!("overwritten using r, this is a long line");
    print!("\r");
    print!("new text to be printed\n");

    println!(
        "printing using formatting with indexes: {0} -> {1} -> {2}",
        "MGH", 12, true
    );

    println!(
        "printing using named vars: name: {name}, experience: {exp}, working: {working}",
        name = "MGH",
        exp = 12,
        working = true
    );

    // =============== Reading input from stdin ===================
    println!("Enter your score:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input data");
    // parsing input string to float
    let score: f32 = input
        .trim()
        .parse()
        .expect("failed to parse input as float number");

    println!("your score is: {}", score);
}
