// traits in rust are similar to interfaces in other languages like Golang.

trait SuperTrait1 {
    fn tell_me_more(&self);
}
trait SuperTrait2 {
    fn tell_me_more_and_more(&self) {
        println!("I am a super trait 2");
    }
}

trait Shape: SuperTrait1 + SuperTrait2 {
    fn area(&self) -> f32;

    // we can also define default implementation for the trait
    fn display(&self) {
        println!("Displaying the shape: area is {}", self.area());
    }
}

trait Renderer {
    fn render(&self) {
        println!("Rendering the shape");
    }
}

struct Square {
    side: f64,
}

// using this syntax we can implement the trait for the struct
impl Shape for Square {
    fn area(&self) -> f32 {
        self.side as f32 * self.side as f32
    }

    // we can also override the default implementation
    fn display(&self) {
        println!("Displaying the square: area is {}", self.area());
    }
}
impl Renderer for Square {}
impl SuperTrait1 for Square {
    fn tell_me_more(&self) {
        println!("I am a super trait 1 for square");
    }
}
impl SuperTrait2 for Square {}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.width as f32 * self.height as f32
    }
}

impl Renderer for Rectangle {}
impl SuperTrait1 for Rectangle {
    fn tell_me_more(&self) {
        println!("I am a super trait 1 for rectangle");
    }
}
impl SuperTrait2 for Rectangle {}

// 1) we can use "trait bounds" to specify that the generic type should implement the trait
fn shape_properties_1<T: Shape>(object: &T) {
    object.area();
    object.display();
}

// 2) we can also use "impl syntax" to specify that the object should implement the trait
fn shape_properties_2(object: &impl Shape) {
    object.area();
    object.display();
}
// 3) we can use multiple trait bounds using + syntax
fn shape_properties_3(object: &(impl Shape + Renderer)) {
    object.area(); // from shape
    object.display(); // from shape
    object.render(); // from renderer
}

// 4) using where clause to specify multiple trait bounds
fn shape_properties_4<T>(object: &T)
where
    T: Shape,
{
    object.area();
    object.display();
}
// 4.1) using where clause to specify multiple trait bounds
fn shape_properties_4_1<T>(object: &T)
where
    T: Shape + Renderer,
{
    object.area();
    object.display();
    object.render();
}

// 6) we can use "impl syntax" in return type as well
fn get_shape() -> impl Shape {
    Square { side: 10.0 }
}

fn get_shape_2() -> impl Shape + Renderer {
    Square { side: 10.0 }
}

fn main() {
    let sq = &Square { side: 10.0 };
    println!("Area of square: {}", sq.area());
    sq.display();

    println!("-------------------");

    let rect = &Rectangle {
        width: 20.0,
        height: 30.0,
    };
    println!("Area of rectangle is {}", rect.area());
    rect.display();

    println!("-------------------");
    shape_properties_1(sq);
    shape_properties_1(rect);

    println!("-------------------");
    shape_properties_2(sq);
    shape_properties_2(rect);

    println!("-------------------");
    shape_properties_3(sq);
    shape_properties_3(rect);

    println!("-------------------");
    shape_properties_4(sq);
    shape_properties_4(rect);

    println!("-------------------");
    shape_properties_4_1(sq);
    shape_properties_4_1(rect);

    println!("-------------------");
    let shape = get_shape();
    shape.display();

    println!("-------------------");
    let shape = get_shape_2();
    shape.display();
    shape.render();
    shape.tell_me_more();
    shape.tell_me_more_and_more();
}
