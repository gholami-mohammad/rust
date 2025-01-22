use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
}; // mpsc: multiple producer, single consumer

fn main() {
    // we can define a channel using this syntax.
    let (tx, rx) = mpsc::channel::<String>();

    /*
    NOTE: Rules of when a channel is closed?
    1- When the Sender is Dropped:
        If all Sender handles for a channel are dropped (i.e., go out of scope or are explicitly dropped),
        the channel is considered closed. This signals to any Receiver that no more messages will be sent on that channel.
    2- Explicit Closure:
        You can explicitly close a channel by dropping all the Sender handles or by using the std::mem::drop function on the sender.
    3- Error on Send/Recv:
        If an error occurs during sending (like the receiver being dropped), subsequent attempts to send will
        result in an error, which can be interpreted as the channel being closed.
        Similarly, if the sender is dropped, receiving operations will fail, indicating closure.
    */

    // now, we create new thread and use tx to send data to the channel.
    // we should use move keyword to move ownership of tx from main to the closure function.
    thread::spawn(move || {
        let username = String::from("kaveh");
        // wait 4 seconds before sending any data
        sleep(Duration::from_secs(4));
        match tx.send(username) {
            Ok(_) => println!("username sent to the channel"),
            Err(_) => println!("failed to send username to the channel"),
        }

        // after exiting from this function, tx will be dropped automatically, so based on rule 1, the channel will be closed.
    });

    println!("waiting to receive any data on the channel");
    // to block the function to wait for receiving data, we can use this syntax:
    // channel is like a queue. it follows FIFO rule.
    match rx.recv() {
        Ok(data) => println!("message received: {}", data),
        Err(e) => println!("error on receiving data: {}", e),
    }

    println!("channel is closed");

    println!("--------------------------------------------------");
    // --------------------------------------------------
    // sending to multiple tx and receiving with one rx

    let (tx, rx) = mpsc::channel::<i32>();
    for i in 0..10 {
        // to be able to use tx in multiple thread, we should create a clone of it
        let cloned_tx = tx.clone();
        thread::spawn(move || {
            sleep(Duration::from_secs(i % 3));
            let _ = cloned_tx.send(i as i32);
            println!("{} sent", i);
        });
    }

    // NOTE: in for loop, we are creating clones of tx. each clone will be dropped when function execution is finished.
    // but the main tx is still alive and we should drop it also.
    drop(tx);

    // we can wait for all senders to be closed:
    for msg in rx {
        println!("message received: {}", msg);
    }

    println!("--------------------------------------------------");
    // --------------------------------------------------
    // using try_recv
    // try_recv function is non blocking function but recv is blocking.
    // we can use try_recv to be able to do some other stuff while waiting for any data on the channel.
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let _ = tx.send("STOP".to_string());
    });

    let mut have_data = false;

    while !have_data {
        match rx.try_recv() {
            Ok(data) => {
                have_data = true;
                println!("data on the channel: {}", data);
            }
            Err(_) => println!("still there is no data"),
        }
    }
}
