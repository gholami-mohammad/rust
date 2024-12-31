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

// category is submodule of product.
// we should have definition here
// and should add declaration in product folder with name category.rs
mod category;
