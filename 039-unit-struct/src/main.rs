// unit structs are structs with no field
// it takes zero memory
struct Admin;

trait Authenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}

// even we implement some other trait for unit structs, they are zero sized
impl Authenticate for Admin {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "admin"
    }
}

fn main() {
    println!("Admin struct size {}", size_of::<Admin>());

    let a = Admin;
    let is_authenticated = a.authenticate("admin", "admin");
    println!("Admin is authenticated: {}", is_authenticated);
}
