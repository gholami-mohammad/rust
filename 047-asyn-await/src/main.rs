use std::time::Duration;

use tokio::time::sleep;

async fn check_service_status(caller: String) -> String {
    println!("fetching data for {}", caller);
    // using tokio sleep instead on standard thread sleep
    sleep(Duration::from_secs(3)).await;
    println!("data loaded for {}", caller);

    String::from("up and running")
}

#[tokio::main]
async fn main() {
    let status_checker = check_service_status(String::from("main"));

    let status = status_checker.await;
    println!("service status: {}", status);

    // we can use tokio spawn function to run concurrent tasks
    let t1 = tokio::spawn(async {
        let s = check_service_status(String::from("t1")).await;
        println!("checking service status using tokio spawn at t1: {}", s);
    });
    let t2 = tokio::spawn(check_service_status(String::from("t2")));
    // we can wait for t1 tokio spawn tasks to be completed.
    match t2.await {
        Ok(data) => {
            println!("result of service status using t2: {}", data);
        }
        Err(_) => {}
    };

    sleep(Duration::from_secs(1)).await;
    t1.await;
}
