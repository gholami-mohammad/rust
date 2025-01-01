use module_exporting::{Category, Order, Product}; // we can import it because of this line in lib.rs: pub use product::Product

use module_exporting::customer::Customer; // we can use customer::Customer because it is a public module
fn main() {
    let p1 = Product::new(
        1,
        String::from("Airpods Pro Gen2"),
        Category::Electronic,
        129.99,
    );
    let p2 = Product::new(
        1,
        String::from("MacBook Pro"),
        Category::Electronic,
        1390.99,
    );

    let customer = Customer::new(
        12,
        String::from("Mohammad"),
        String::from("mgh@example.com"),
    );

    let order = Order::new(88, customer, vec![p1, p2]);
    let total = order.total_bill();

    println!("Total Bill: {}", total);
}
