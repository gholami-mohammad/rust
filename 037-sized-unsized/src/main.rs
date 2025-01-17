struct Point {
    x: i32,
    a: bool,
}
fn main() {
    // ----------------------------------
    //         Sized Types
    // ----------------------------------
    println!("size of i32 is {}", size_of::<i32>());
    println!("size of u8 is {}", size_of::<u8>());
    println!("size of bool is {}", size_of::<bool>());
    println!("size of array with 3 elements is {}", size_of::<[i32; 3]>());

    // i32 = 4, u8=1 => 5 bytes total
    // but compiler may add some padding bytes and resulting to a bigger size
    println!("size of tuple (i32, u8) is {}", size_of::<(i32, u8)>());

    // bool=1 + i32=4 => 5
    // but getting 8 in total => 3 padding bytes
    println!("size of Point {}", size_of::<Point>());

    // references need one machine word. it can be different based on architecture.
    // in my mac  book m3 chip it is 8
    println!("size of references: {}", size_of::<&i32>());

    println!("My machine word size is {}", size_of::<&()>());

    // unit type is represented by ()
    // it takes no memory size
    println!("Unit Type Size {}", size_of::<()>());

    println!("function size: {}", size_of::<fn(i32) -> i32>());

    // ----------------------------------
    //         Unsized Types
    // ----------------------------------
    // take a look at deref coercion and unsized coercion :)
}
