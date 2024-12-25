fn main() {
    // Variables can be declared using the `let` keyword
    println!("====== Defining vars ======");
    let x: i16 = 90;
    let y = 21;
    let z = 6.4;
    let a: f32 = 3.14;

    println!("x is {x}, y is {y}, z is {z}, a is {a}"); // formatting
    println!("x is {}, y is {}, z is {}, a is {}", x, y, z, a); // formatting

    // variable mutability
    // variables are immutable by default
    let a = 21;
    println!("a is {}", a);
    // a = 33// error: cannot assign twice to immutable variable `a`

    // defining mutable variables
    let mut b = 21;
    println!("b is {}", b);
    b = 33;
    println!("b is {}", b);

    // variable scope
    let c = 21;
    {
        println!("====== Scope ======");
        let d = 33;
        println!("c is {}, d is {}", c, d);
    }

    let v = 30;
    {
        println!("v in main scope is printed in inner scope {}", v);
        let v = 40; // shadowing
        println!("v in inner scope is {}", v);
    }
    println!("v in main scope is {}", v);

    // we can not access d here

    // shadowing
    println!("====== Shadowing ======");
    let e = 21;
    println!("e is {}", e);
    let e = 44;
    println!("e is {}", e);
    let e = e + 99;
    println!("e is {}", e);
    let e: &str = "mgh"; // shadowing with different type
    println!("e is {}", e);

    // constants
    println!("====== Constants ======");
    const MAX_SCORE: i16 = 255;
    println!("MAX_SCORE is {}", MAX_SCORE);
}
