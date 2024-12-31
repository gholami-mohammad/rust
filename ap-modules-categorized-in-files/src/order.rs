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
