use std::sync::Mutex;

fn main() {
    // we can create new mutex like this.
    let m = Mutex::new(10);

    {
        let mut num = m.lock().unwrap();
        println!("before lock 1 before change = {}", num);
        // mutate the mutex value
        *num = *num * 10;
        println!("before lock 1 after change = {}", num);

        // here, as soon as we go out of this scope, m will be unlocked.
        // this will be done automatically and do not need to unlock it manually
    }

    let lock_2 = m.lock().unwrap();
    println!("lock 2 = {}", lock_2);

    // we can not lock m again in this code block if only we drop it manually:
    drop(lock_2); // this will unlock m

    let lock_3 = m.lock().unwrap();
    println!("lock 3 = {}", lock_3);
}
