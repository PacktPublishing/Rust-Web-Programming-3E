use std::{thread, time};


async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    // replace with tokio::time::sleep(two_seconds).await; for non-blocking sleep
    thread::sleep(two_seconds);
    return 2
}



#[tokio::main(worker_threads = 1)]
async fn main() {
    // just calling the future
    let now = time::Instant::now();
    let future_one = do_something(1);
    let outcome = future_one.await;
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);

    // sleeping before calling the future
    let now = time::Instant::now();
    let future_one = do_something(1);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    let outcome = future_one.await;
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);

    // spawning the future then sleeping
    let now = time::Instant::now();
    let future_one = tokio::spawn(do_something(1));
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    let outcome = future_one.await.unwrap();
    println!("time elapsed {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);

    // spawning multiple futures before sleeping
    let future_one = tokio::spawn(do_something(1));
    let future_two = tokio::spawn(do_something(2));
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    let _ = future_one.await.unwrap();
    let _ = future_two.await.unwrap();
}
