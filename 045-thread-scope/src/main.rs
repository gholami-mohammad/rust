use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // scoped threads allow us to borrow local variables more effectively.
    // it also wait the execution of code until all sub spawn functions executed.

    let mut ids = vec![1, 2, 3];

    thread::scope(|scope| {
        // look at the spawn, we did not use move keyword to move ids inside the thread.
        // this allow us to access ids variable after all scopes.
        // now ids, is borrowed immutably from main.
        scope.spawn(|| {
            sleep(Duration::from_secs(3));
            println!("printing from spawn 1: {:?}", ids);
        });

        scope.spawn(|| {
            sleep(Duration::from_secs(1));
            println!("printing from spawn 2: {:?}", ids);
        });
    });

    // code wont be executed here until all scope spawns finished.
    // rust will make sure that there is no valid pointer to ids up to this line anymore.
    ids.push(4);
    println!("ids  after scope: {:?}", ids);
}
