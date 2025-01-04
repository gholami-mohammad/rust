trait Shape {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
    line_width: u64,
    color: String,
}
impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

struct Rectangle {
    width: f64,
    height: f64,
    line_width: u64,
    color: String,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// STATIC DISPATCH
// this type of generic is of type static dispatch.
// in static dispatch, the compile will generate the code for each type that is passed to the function.
// in this case, because we called shape_properties one time with Square and one time with Rectangle, the compiler will generate two versions of the function.
// for example: shape_properties_square and shape_properties_rectangle.
// then replace the calls to shape_properties with the correct function.
// this is static dispatch and there is no performance penalty.
// what we covered in previous examples: 023-generics and 024-trait, are examples of static dispatch.
fn shape_properties<T>(shape: T) {}

// DYNAMIC DISPATCH
// in dynamic dispatch, the compiler wont generate code for each type at compile time.
// Box is a smart pointer which is a pointer to some heap allocated memory.
// dyn keyword is used to specify the trait object.
// dyn should be behind a pointer. Box is a pointer, so we can use dyn with Box.
// we can also use dyn with reference, like &dyn Shape.
// for more info, look at build_shape function.
fn shape_properties_dynamic(shape: Box<dyn Shape>) {}

// in this function, we are returning both Square and Rectangle based on the input.
// in Static Dispatch, we can't return different types from the function.
// but using Box<dyn Shape>, we can return different types.
// if this function was like this: fn build_shape(sides: Vec<f64>) -> impl Shape
// we will have a compile error based on this function body.
fn build_shape(sides: Vec<f64>) -> Box<dyn Shape> {
    if sides.len() == 1 {
        Box::new(Square {
            side: sides[0],
            line_width: 2,
            color: String::from("red"),
        })
    } else {
        Box::new(Rectangle {
            width: sides[0],
            height: sides[1],
            line_width: 2,
            color: String::from("blue"),
        })
    }
}

trait Draw {
    fn draw_object(&self);
}

fn main() {
    println!("Hello, world!");

    let square = Square {
        side: 10.0,
        line_width: 2,
        color: String::from("red"),
    };
    shape_properties(square);

    let rect = Rectangle {
        width: 10.0,
        height: 20.0,
        line_width: 2,
        color: String::from("blue"),
    };
    shape_properties(rect);
}
