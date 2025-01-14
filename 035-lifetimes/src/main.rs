// -----------------------
// Generic lifetimes
// -----------------------

// returning value lifetime is equal to shortest  variable lifetime.
// in this example, i and j and returned value have 'a lifetime.
// so, the lifetime of returned value will be equal to i lifetime if it has a shorter lifetime than j and vise versa.
fn picking_int<'a>(i: &'a i32, j: &'a i32) -> &'a i32 {
    if i < &100 {
        i
    } else {
        j
    }
}

// this function will only return i and returning value is not depends of j.
// so we can remove lifetime specifier from j.
fn picking_int_2<'a>(i: &'a i32, j: &i32) -> &'a i32 {
    i
}

// STATIC LIFETIME
// it is a special lifetime that defines a reference that can live for the  entire duration of the application
fn using_static_lifetime() -> &'static i32 {
    // we can not return reference to x here because its lifetime will be ended after  the  function block.
    // to solve this issue, we should define a static lifetime for it. (USE STATIC LIFETIME WITH CARE)
    /*
     * let x = 129;
     * return &x;
     */

    let x: &'static i32 = &129;

    x
}

fn main() {
    // generic lifetime examples

    // #1
    let i = 120;
    let j = 90;
    // here, i and j have same lifetime until the main function ends. so lifetime of k will be same.
    let k = picking_int(&i, &j);
    println!("ex 1: {}", k);

    // #2
    let i = 44;
    let k: &i32;
    {
        let j = 33;

        // here, k lifetime is same as j. because j is only valid until the end of this block
        // so we can NOT print k out  of this block otherwise  we will get compile error.
        k = picking_int(&i, &j);
        println!("ex 2: {}", k);
    }

    // #3
    let i = 44;
    let k: &i32;
    {
        let j = 33;

        // k lifetime is equal to i because it does not depend of j.
        // so we can access k outside this block
        k = picking_int_2(&i, &j);
    }
    println!("ex 3: {}", k);

    // #4
    let k = using_static_lifetime();
    println!("ex 4 (static): {}", k);
}
