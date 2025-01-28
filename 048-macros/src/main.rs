/*

What are Rust Macros?

Rust macros are a powerful feature of the Rust programming language that allow for metaprogramming. They essentially work as a way to write code that writes other code. There are two primary types of macros in Rust:

1) Declarative Macros (also known as macro_rules! macros): These are defined using the macro_rules! macro. They operate by matching patterns in your code and expanding them into new code based on those patterns. They are essentially a way to reduce boilerplate by automating repetitive coding patterns.

2) Procedural Macros: These are more complex and allow for custom behavior during compilation. They can be thought of as functions that receive Rust code as input and output Rust code. There are three kinds of procedural macros:

3) Custom derive macros: Used to implement traits for types automatically.

4) Attribute-like macros: Applied to expressions or items, similar to attributes but with custom logic.
Function-like macros: Look like function calls but are expanded by Rust's compiler.

How Useful Are They?

- Code Reusability: Macros help in avoiding code duplication by allowing you to define complex constructs once and reuse them across your codebase. This can significantly reduce the amount of code you write and maintain.

- Domain-Specific Language (DSL): With macros, you can essentially create a mini-language within Rust tailored to your specific needs or problem domain. This can make the code more expressive and easier to understand for those familiar with the domain.

- Custom Syntax: Macros can extend Rust's syntax. For instance, if you want a particular pattern or structure to be used frequently in your project, you can define a macro that simplifies or customizes how this structure is written.
Performance: Since macros are expanded at compile time, there's no runtime overhead from their use. This means you get all the benefits of code reuse without any performance penalty.

- Error Handling: Macros can enforce coding standards or patterns by ensuring that certain conditions or structures are always met or followed, which can help in reducing errors.
Learning Curve: While macros offer immense power, they also come with a steeper learning curve. Writing good macros can be challenging due to the complexities of ensuring they interact correctly with Rust's type system and syntax.

- Debugging: Macro expansions can sometimes make debugging more complicated since the code you write is not exactly what the compiler sees after macro expansion.

In summary, Rust macros are incredibly useful for improving code quality, reducing redundancy, and enabling more expressive code constructs. However, they require careful use because of their complexity and potential to obscure the actual runtime code. For beginners, it's often advised to learn the basics of Rust before diving into the world of macros, but for more advanced users, macros are a tool that can significantly enhance productivity and code clarity.
*/

/*
we can install cargo-expand to show our code and macro result same as what is seen in compile

```
cargo install cargo-expand
cargo-expand expand
```
*/

// ------------------------
// declarative macro
// ------------------------
macro_rules! my_macro {
    () => {
        "nothing selected" // it is better to omit ; at the end of expression.
    };
    (we can have any pattern) => {
        println!("we can have any pattern is used :)");
    };
    // we can use $name: expr syntax to define place holders of expressions.
    // loot at this macro call usage. Also look at the separator we used: ,
    (sum: $a: expr, $b: expr) => {
        $a + $b
    };
    (mul: $a: expr; $b: expr) => {
        $a * $b
    };
}

// ------------------------
// capturing types
// ------------------------
// to capture type in macro, we can use "$type_name: ty" to send types into macro.
macro_rules! input {
    ($t: ty) => {{
        // in this example, we will read data from stdin and parse it to the given type
        let mut n = String::new();
        println!("enter desired input:");
        std::io::stdin()
            .read_line(&mut n)
            .expect("failed to read input");

        let m: $t = n.trim().parse().expect("invalid input");
        m
    }};
}

// ------------------------
// identifiers
// ------------------------
// we can declare an identifier to be able to have same variable from outside and inside the macro
macro_rules! using_identifier {
    ($var: ident) => {
        $var = $var + 10
    };
}

macro_rules! add_as {
    ($a: expr, $b: expr, $t: ty) => {
        $a as $t + $b as $t
    };
}

// this macro will create a simple function for us
macro_rules! create_function {
    ($fn: ident, $input_name: ident, $input_type: ty, $output_type: ty) => {
        fn $fn($input_name: $input_type) -> $output_type {
            $input_name
        }
    };
}

create_function!(my_fn, x, i32, i32);

// ------------------------
// repeating pattern
// ------------------------
// if we need a vardic of parameters, we can use repeating pattern.
// we can use this syntax: $($var_name: expr)DELIMITATE_CHAR *|+|?
// DELIMITATE_CHAR can be any char. default is space.
// * means: zero or more
// + means: one or more
// ? means: zero or one
// for example: "$($values: expr); * " means: repeated values which separated by ; and can be zero, one or more

// this macro will accept multi string values, and concat them to gether and return the final string.
macro_rules! str_concat {
    ($($str: expr), *) => {{
        let mut s = String::new();

        $(s.push_str($str);)*
        s
    }};
}

fn main() {
    let n = my_macro!();
    println!("my_macro with no pattern: {}", n);

    // this will use the 2nd macro pattern
    my_macro!(we can have any pattern);

    let sum = my_macro!(sum: 10, 30);
    println!("sum of 10 and 30: {}", sum);

    let mul = my_macro!(mul: 4; 5);
    println!("mul result of 4*5: {}", mul);

    // we can also call macro using () or [] or {}
    println!("{}", my_macro!());
    println!("{}", my_macro![]);
    println!("{}", my_macro! {});
    println!("{}", my_macro! {sum: 9, 80});
    println!("{}", my_macro! [mul: 9; 80]);

    // ------------------------
    // capturing types examples
    // ------------------------
    let age = input!(i32);
    println!("age from input is: {}", age);

    let score = input!(f32);
    println!("your score is {}", score);

    let sum_of_int_and_float = add_as!(32.3, 33, f32);
    println!("sum of 32.3 and 33 = {}", sum_of_int_and_float);

    // ------------------------
    // identifiers examples
    // ------------------------
    let mut x = 12;
    println!("x value before using_identifier macro: {}", x);
    using_identifier!(x);
    println!("x value after using_identifier macro: {}", x);

    // calling generated function using create_function
    let m = my_fn(10);
    println!("m: {}", m);

    // ------------------------
    // repeating pattern examples
    // ------------------------
    let nothing = str_concat!();
    println!("repeating: null: {}", nothing);

    let one = str_concat!("Kaveh");
    println!("repeating: one: {}", one);

    let two = str_concat!("Kaveh", "Zari");
    println!("repeating: two: {}", two);

    let three = str_concat!("Kaveh", "Zari", "Gholami");
    println!("repeating: three: {}", three);
}
