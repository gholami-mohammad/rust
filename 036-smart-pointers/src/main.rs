//#region box
// --------------------------------------
//          Box Smart Pointer
// --------------------------------------
// by default, rust store variables on the stack. we can use Box smart pointer to store values in heap.

/*
look at this example:

enum List {
    Cons(i32, List),
    Nil,
}

this code will produce this compile error: recursive type `List` has infinite size. Because rust can not determine the stack size required to store the list data.
to solve this issue, we can use heap. Using Box, Rc or simple reference &, we  can solve the issue:
*/

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListImproved {
    Cons(i32, Option<Box<ListImproved>>),
}

fn box_smart_pointer_examples() {
    let x = 4.23; // x will be stored in stack
    let y = Box::new(x); // y will be stored in heap
    let z = &x; // z is pointer to x but it stored in stack

    let list_1 = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list 1 is: {:?}", list_1);

    let list_2 = ListImproved::Cons(
        1,
        Some(Box::new(ListImproved::Cons(
            2,
            Some(Box::new(ListImproved::Cons(3, None))),
        ))),
    );
    println!("list 2 is: {:?}", list_2);

    // 2ND use case of Box
    // Box smart pointer is also beneficial when going to transfer ownership of a large amount of data instead of copying it
    #[derive(Debug)]
    struct HugeData;
    #[derive(Debug)]
    struct SmallData;
    trait Storage {}
    impl Storage for HugeData {}
    impl Storage for SmallData {}

    let data_1 = HugeData;
    let data_2 = Box::new(HugeData);

    // here, ownership of data_1 is moved to data_3 and all stack data will be copied
    // copying of a huge data will consuming more time
    let data_3 = data_1;

    // here, ownership of data_2 is also transfer to data_4 but here, only the reference data address will be copied
    // and it will be much more faster than the prev example.
    let data_4 = data_2;

    let data_5 = Box::new(SmallData);
    let collection: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];
}
//#endregion box

//#region RC
/*
RC = Reference Counting

consider we have these lists
a = 1,2,3,NIL
b = 5, &a
c = 6, &a

a is referenced in both b and c and its data should be valid until one of b or c is valid.
 */

enum ListWithRC {
    Cons(i32, Option<Rc<ListWithRC>>),
}
fn rc_example() {
    let a = Rc::new(ListWithRC::Cons(
        1,
        Some(Rc::new(ListWithRC::Cons(2, None))),
    ));
    println!("Reference count after a: {}", Rc::strong_count(&a));

    let b = ListWithRC::Cons(3, Some(Rc::clone(&a))); // here Rc::clone does not clone all data in new reference. it create new reference to 'a' smart pointer
    println!("Reference count after b: {}", Rc::strong_count(&a));

    let c = ListWithRC::Cons(4, Some(Rc::clone(&a)));
    println!("Reference count after c: {}", Rc::strong_count(&a));
    // references to a will be dropped in LIFO. means, when rc_example finished, first, c will be cleared and the first reference to a will be removed
    // then b will be dropped and the reference to a will be 1
    // after all, a  will be dropped and references to a  will be 0
    // now, compiler knows that it  can delete that data.

    {
        let d = ListWithRC::Cons(5, Some(Rc::clone(&a)));
        println!("Reference count after d: {}", Rc::strong_count(&a));

        let e = ListWithRC::Cons(5, Some(Rc::clone(&a)));
        println!("Reference count after e: {}", Rc::strong_count(&a));
    }
    println!(
        "Reference count after inner scope: {}",
        Rc::strong_count(&a)
    );
}
// Now, a has 3 owners. a, b, and c.
//#endregion RC

fn main() {
    box_smart_pointer_examples();
    rc_example();
}
