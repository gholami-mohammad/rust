use std::thread;

fn main() {
    println!("before starting new thread");

    // we should use thread::spawn to create new thread
    // the returning value from thread is a JoinHandler. we can use it to wait for the thread to be completed.
    let t1 = thread::spawn(|| println!("this is printed from thread"));

    println!("this statement is placed after thread but may run before it");

    // we can use join method to wait for the thread until it is done
    let _ = t1.join();

    println!("this print is after thread completion");

    // ownership in threads
    let x = String::from("some string");

    // to use x inside t2 closure function, we can use move keyword
    // to move the ownership of x from main to t2
    let t2 = thread::spawn(move || {
        println!("x value from main inside t2 using move keyword {}", x);
    });

    let _ = t2.join();

    let z = String::from("z string");
    // we can also move the ownership using any closure that implements FnOnce
    let t3 = thread::spawn(|| {
        // this assignment will lead to having FnOnce here
        // and ownership of z will be moved to t3
        let y = z;

        println!("y value from z moved ownership: {}", y);
    });
    let _ = t3.join();
}
