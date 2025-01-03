// integration tests should be placed in the tests directory
// we can only test the public API of the library crate
// we can run integration tests of a specific file by running `cargo test <file_name>`

use integration_test::{customer::Customer, Category, Order, Product};

mod helpers; // we should add helpers module define here to be able to access its implementation.

#[test]
fn total_bill_without_discount() {
    helpers::setupConfig(); // -> we can call setupConfig function of helpers module.

    let p = Product::new(1, "Product 1".to_string(), Category::Electronic, 100.0);
    let p2 = Product::new(1, "Product 2".to_string(), Category::Electronic, 200.0);
    let c = Customer::new(1, "MGH".to_string(), "gholami@example.com".to_string());
    let order = Order::new(1, c, vec![p, p2]);
    assert_eq!(order.total_bill(), 330.0); // 300 + 10% tax
}

#[test]
fn total_bill_with_discount() {
    let p = Product::new(1, "Product 1".to_string(), Category::Electronic, 700.0);
    let p2 = Product::new(1, "Product 2".to_string(), Category::Electronic, 150.0);
    let p3 = Product::new(1, "Product 3".to_string(), Category::Electronic, 250.0);
    let p4 = Product::new(1, "Product 4".to_string(), Category::Electronic, 280.0);
    let p5 = Product::new(1, "Product 5".to_string(), Category::Electronic, 300.0);
    let c = Customer::new(1, "MGH".to_string(), "mgh@example.com".to_string());

    let order = Order::new(1, c, vec![p, p2, p3, p4, p5]);
    let total = order.total_bill();
    assert_eq!(format!("{:.2}", total), "1570.80"); // 700+150+250+280+300 = (1680  + 10% tax) - 15% discount
}
