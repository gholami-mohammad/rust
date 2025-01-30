struct User {
    name: String,
    age: u8,
}
impl User {
    fn say_my_name(&self) -> String {
        // we can use todo!() macro for any part of code
        // that we want to complete it in the future.
        // if we call a function containing todo macro,
        // it will return with a panic.
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
