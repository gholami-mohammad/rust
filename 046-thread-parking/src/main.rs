use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // we can use thread parking to temporary pause a thread for processing.
    // we can unpark the parked thread in other threads or the parent.
    let t1 = thread::spawn(|| {
        println!("thread 1 before parking");
        thread::park();
        println!("thread 1 unparked");
    });

    sleep(Duration::from_secs(2));
    // this will let t1 to continue processing
    t1.thread().unpark();
    let _ = t1.join();
    println!("t1 done.");

    // we can use park_timeout to call unpark automatically after the duration if unpark is not called anywhere.
    let t2 = thread::spawn(|| {
        println!("thread 2 before parking");
        thread::park_timeout(Duration::from_secs(2));
        println!("thread 2 unparked");
    });

    // because of not calling unpark for t2, it will be automatically unparked after 2 seconds.
    t2.join();
    println!("t2 done");
}
