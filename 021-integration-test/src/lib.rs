pub use order::Order;
pub use product::{Category, Product}; // using this syntax expose Product struct and now we can import it in main

mod product {
    pub use category::Category; //  re-exporting from submodule to product

    pub struct Product {
        id: u64,
        name: String,
        category: Category,
        price: f32,
    }

    impl Product {
        pub fn new(id: u64, name: String, category: Category, price: f32) -> Product {
            Product {
                id,
                name,
                category,
                price,
            }
        }
        pub fn calculate_tax(&self) -> f32 {
            self.price * 0.1
        }

        pub fn get_final_price(&self) -> f32 {
            self.price + self.calculate_tax()
        }
    }

    mod category {
        pub enum Category {
            Electronic,
            Home,
            Sports,
        }
    }
}

pub mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }

    impl Customer {
        pub fn new(id: u64, name: String, email: String) -> Customer {
            Customer { id, name, email }
        }
    }
}

mod order {
    use crate::customer::Customer;
    use crate::product::Product;

    pub struct Order {
        id: u64,
        customer: Customer,
        products: Vec<Product>,
    }

    impl Order {
        pub fn new(id: u64, customer: Customer, products: Vec<Product>) -> Order {
            Order {
                id,
                customer,
                products,
            }
        }

        pub fn calculate_discount(&self) -> f32 {
            if self.products.len() >= 5 {
                return 0.15;
            } else {
                return 0.0;
            }
        }

        pub fn total_bill(&self) -> f32 {
            let mut total: f32 = 0.0;
            for p in &self.products {
                total += p.get_final_price();
            }

            total -= (self.calculate_discount() * total);
            if total < 0.0 {
                return 0.0;
            }

            return total;
        }
    }
}
