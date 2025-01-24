use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep, JoinHandle},
    time::Duration,
};

struct File {
    text: Vec<String>,
}

fn main() {
    // Arc(Atomic Reference Counting) is a smart pointer which allow us having shared resource between multiple threads.
    // Shared Ownership: Unlike Rc (Reference Counting), which is not thread-safe, Arc can be used in a multithreaded environment because it uses atomic operations to manage the reference count, ensuring thread safety.
    let file = Arc::new(Mutex::new(File { text: vec![] }));

    // collect all thread handlers to be able to wait for all of them.
    let mut thread_handlers: Vec<JoinHandle<()>> = vec![];
    for i in 0..10 {
        let cloned_file = Arc::clone(&file);

        let t_handler = thread::spawn(move || {
            // random sleep
            sleep(Duration::from_secs(i % 3));

            let mut f = cloned_file.lock().unwrap();
            f.text.push(format!("text from {}", i));
        });

        thread_handlers.push(t_handler);
    }

    // to make sure all threads is completed we can use join on all of them
    for h in thread_handlers {
        let _ = h.join();
    }

    // displaying file content
    let f_lock = file.lock().unwrap();
    for line in &f_lock.text {
        println!("{}", line);
    }
}
