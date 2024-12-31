// If we are working with a library crate, we should create lib.rs
// if we are working with a binary crate, we should create main.rs

// To visualize cargo modules, first install cargo-modules:  cargo install cargo-modules
// then use this command to visualize modules:
//    1: cargo modules structure --lib => for lib crates
//    1: cargo modules structure --bin => for binary crates

pub mod product {
    // category module is a submodule of product
    // so we can use it with relative path
    use category::Category;

    // use pub keyword to make a struct public
    // by default everything in a module is private
    pub struct Product {
        id: u64,
        name: String,
        category: Category,
        price: f32,
    }

    impl Product {
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

mod customer {
    pub struct Customer {
        id: u64,
        name: String,
        email: String,
    }
}

mod order {
    // customer module should be used using relative path starting with crate
    // crate:: is root module of project
    use crate::customer::Customer;
    use crate::product::Product;

    struct Order {
        id: u64,
        customer: Customer,
        products: Vec<Product>,
    }

    impl Order {
        pub fn calculate_discount(&self) -> f32 {
            if self.products.len() >= 5 {
                return 0.1;
            } else {
                return 0.0;
            }
        }

        pub fn total_bill(&self) -> f32 {
            let mut total: f32 = 0.0;
            for p in &self.products {
                total += p.get_final_price();
            }

            total -= self.calculate_discount();
            if total < 0.0 {
                return 0.0;
            }

            return total;
        }
    }
}
