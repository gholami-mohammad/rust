// 1) To open documentation of your project run `cargo doc --open`
// 2) To separate documentation form comments, use /// instead of //
// 3) If adding a documentation to a private module, function or struct, it wont be shown in the documentation html file
// 4) To include documentation of private parts, use `cargo doc --document-private-items --open`
// 5) We can run documentation tests by running `cargo test --doc`. this will test the code examples in the documentation.
// 6) To add documentation to crate root, use //! syntax
// 7) To add section in documentation, use //! # Section Name or /// # Section Name

//! # This a section
//!  This is a documentation for the crate
//! ## This is a subsection

pub use order::Order;
pub use product::{Category, Product};

/// this a documentation for the product module which is private
mod product {
    pub use category::Category;

    /// Product struct represents a product in the store
    pub struct Product {
        pub id: u64,
        name: String,
        category: Category,
        price: f32,
    }

    impl Product {
        /// # Example
        /// ```
        /// use documentation::{Category, Product};
        /// let product = Product::new(1, "Laptop".to_string(), Category::Electronic, 1000.0);
        /// assert_eq!(product.id, 1);
        /// ```
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
        /// category enum represents the category of the product
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
