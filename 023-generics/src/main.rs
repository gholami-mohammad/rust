#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// what is Monomorphization? It is the process of turning generic code into specific code by the compiler.
// due to monomorphization, there is no runtime cost for using generics. It is all done at compile time.
impl<T, U> Point<T, U> {
    pub fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

// this kind of implementation is only for Point<i32, i32>
// this is called specialization
impl Point<i32, i32> {
    pub fn printing(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

// adding generics to a function
fn do_something<T, U>(x: T, y: U) -> Result<U, String> {
    // do something
    Ok(y)
}

fn main() {
    let p0 = Point { x: 1, y: 2 };
    println!("p0: {:?}", p0);
    // printing function is only available for Point<i32, i32> due to specialization
    p0.printing();

    let p1 = Point { x: 3.2, y: 23.1 };
    println!("p1: {:?}", p1);
    let p2 = Point { x: 3.2, y: 23 };
    println!("p2: {:?}", p2);

    let p3 = Point::new(2.3, 22);
    println!("p3: x: {}, y: {}", p3.x, p3.y);
}
