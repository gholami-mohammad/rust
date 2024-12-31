fn main() {
    // Scalar types (Primary types)

    // integers
    let i: i8 = 1; // 8-bit signed integer
    let i: i16 = 1; // 16-bit signed integer
    let i: i32 = 1; // 32-bit signed integer
    let i: i64 = 1; // 64-bit signed integer
    let i: i128 = 1; // 128-bit signed integer
    let i: isize = 1; // architecture dependent signed integer

    // unsigned integers
    let u: u8 = 1; // 8-bit unsigned integer
    let u: u16 = 1; // 16-bit unsigned integer
    let u: u32 = 1; // 32-bit unsigned integer
    let u: u64 = 1; // 64-bit unsigned integer
    let u: u128 = 1; // 128-bit unsigned integer
    let u: usize = 1; // architecture dependent unsigned integer

    // floating point numbers
    let f: f32 = 1.0; // 32-bit floating point number
    let f: f64 = 1.0; // 64-bit floating point number (default type is f64)

    // characters
    let c: char = 'm'; // 4-byte Unicode scalar value

    // booleans
    let b: bool = true; // true or false

    //============ type aliases ============
    type Age = u8;
    let my_age: Age = 35;
    println!("My age is {my_age}");

    type Grade = char;
    let tv_grade: Grade = 'A';
    println!("tv grade is {}", tv_grade);

    //============ type conversion ============
    let a: i32 = 10;
    let b: f64 = a as f64;
    println!("a: {}, b: {}", a, b);

    let a: u8 = 99;
    let ch: char = a as char; // we can convert u8 to char but not i8 because char is a Unicode scalar value
    println!("a: {}, ch: {}", a, ch);

    // ================================================
    // ============ Compound types ====================
    // ================================================

    // ============ string ====================
    // strings : &str (string slice) & String

    // &str (string slice) are immutable and fixed size
    // we can not modify the string slice
    let fixed_str: &str = "this is a string with fixed size and immutable";
    println!("{}", fixed_str);

    // String are mutable and growable
    // the mut keyword is not required because String is mutable by default
    let mut growable_str: String = String::from("we can modify this string ");
    println!("{}", growable_str);
    growable_str.push('M');
    growable_str.push_str(" new string added");
    println!("{}", growable_str);

    // ============ array ====================
    // arrays : [T; N]
    // arrays have fixed size and can not grow
    let arr: [i8; 4] = [1, 3, 5, 8];

    // accessing array element
    println!("array first index: {}", arr[1]);
    //printing compound types:
    println!("full array: {:?}", arr);

    // defining array with default values
    let arr2: [i32; 10] = [5; 10]; // all 10 elements will have value of 5
    println!("array with default values: {:?}", arr2);

    // defining mutable array
    let mut marr: [i8; 10] = [0; 10];
    println!("mutable array init: {:?}", marr);
    marr[4] = 120;
    println!("mutable array after update: {:?}", marr);

    // ============ vector ====================
    // vectors : Vec<T>
    // vectors are growable and mutable
    let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("vector1: {:?}", vec1);
    let mut vec2: Vec<i32> = vec![4, 2, 5, 3]; // mut is optional. vectors are mutable by default
    vec2[1] = 123;
    vec2.push(980);
    println!("vector2: {:?}", vec2);

    let mut vec3: Vec<&str> = Vec::new(); // here mut is required because we are using Vec::new() method
    vec3.push("hello");
    vec3.push("world");
    println!("{:?}", vec3);

    // ============ tuple ====================
    // tuples : (T1, T2, T3, ...)
    // tuples can have different types of elements
    let tup1: (&str, i32, &str, bool) = ("Mohamad", 1368, "Software Engineer", true);
    println!("tuple1: {:?}", tup1);
    let name = tup1.0;
    let birth_year: i32 = tup1.1;
    println!("name: {}, birth year: {}", name, birth_year);

    // defining multiple variables from all elements of tuple
    let (first_name, b_year, job, is_active) = tup1;
    println!(
        "first name: {}, birth year: {}, job: {}, is active: {}",
        first_name, b_year, job, is_active
    );

    // empty tuple
    let emp: () = (); // this tuple does not consume any memory
    println!("empty tuple: {:?}", emp);

    type User = (i32, String, bool);
    let user1: User = (1, String::from("Mohamad"), true);
    println!("user1: {:?}", user1);
    let user2: User = (2, String::from("Ali"), true);
    println!("user2: {:?}", user2);
}
