use std::{
    sync::{Arc, Barrier},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // A barrier is useful when you need all threads to reach a point before any thread can proceed further.
    // now, assume we have multiple thread like this:

    // we can create new barrier like this.
    // this allow us having 3 wait handler.
    let sync_barrier = Arc::new(Barrier::new(3));

    let b1 = sync_barrier.clone();
    let h1 = thread::spawn(move || {
        sleep(Duration::from_secs(3));
        println!("h1 before barrier");

        // if all sync_barrier counter reached the allocated count, rest of this code will be executed.
        b1.wait();

        println!("h1 after barrier.");
    });

    let b2 = sync_barrier.clone();
    let h2 = thread::spawn(move || {
        sleep(Duration::from_secs(1));
        println!("h2 before barrier");

        // if all sync_barrier counter reached the allocated count, rest of this code will be executed.
        b2.wait();

        println!("h2 after barrier.");
    });

    let b3 = sync_barrier.clone();
    let h3 = thread::spawn(move || {
        sleep(Duration::from_secs(2));
        println!("h3 before barrier");

        // if all sync_barrier counter reached the allocated count, rest of this code will be executed.
        b3.wait();

        println!("h3 after barrier.");
    });

    let _ = h1.join();
    let _ = h2.join();
    let _ = h3.join();
}
