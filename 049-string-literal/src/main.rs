fn main() {
    let s = "this is a string";
    println!("s: {}", s);
    // using raw string, we do not need to escape characters
    let s2 = r"this is raw string as you can see, \n is escaped";
    println!("s2: {}", s2);
    // using r#, we can define raw string to escape any kind of char until we reach at ending # char
    let s3 = r#"raw string with escaped " inside the string"#;
    println!("s3: {}", s3);
    // we can use any number of # for beginning and same number of # for ending the string
    let s4 = r##"raw string with 2 # sign we can escape "# also \n \r \t also escaped"##;
    println!("s4: {}", s4);

    let some_json = r#"{
        "name": "kaveh",
        "age": 35
    }"#;
    println!("some_json:\n {}", some_json);
}
