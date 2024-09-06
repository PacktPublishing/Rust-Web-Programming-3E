use std::{future::Future, panic::catch_unwind, thread};
use std::time::{Duration, Instant};
use std::pin::Pin;
use std::task::{Context, Poll};

use async_task::{Runnable, Task};
use futures_lite::future;
use once_cell::sync::Lazy;


/// Spawns an async task by sending it to a thread.
/// 
/// # Arguments
/// * `future` - The future to spawn.
/// 
/// # Returns
/// A task handle that can be awaited on.
fn spawn_task<F, T>(future: F) -> Task<T>
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    static QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        thread::spawn(move || {
            while let Ok(runnable) = rx.recv() {
                let _ = catch_unwind(|| runnable.run());
            }
        });
        tx
    });
    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    return task
}


/// An async sleep future that does not block the thread.
/// 
/// # Fields
/// * `start_time` - The time the sleep started.
/// * `duration` - The duration of the sleep.
struct AsyncSleep {
    start_time: Instant,
    duration: Duration,
}
impl AsyncSleep {

    /// Creates a new async sleep future.
    /// 
    /// # Arguments
    /// * `duration` - The duration of the sleep.
    /// 
    /// # Returns
    /// The new async sleep future.
    fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
        }
    }
}


impl Future for AsyncSleep {
    type Output = ();

    /// Polls the async sleep future.
    /// 
    /// # Arguments
    /// * `cx` - The context to poll the future in.
    /// 
    /// # Returns
    /// The result of polling the future.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) 
        -> Poll<Self::Output> {
        let elapsed_time = self.start_time.elapsed();
        if elapsed_time >= self.duration {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }            
    }
}


/// Runs multiple async sleeps squentially to demonstrate how async tasks are polled.
/// 
/// # Arguments
/// * `label` - The label of the sleep for printing purposes to trace the progress.
async fn sleeping(label: u8) {
    println!("sleeping {}", label);
    AsyncSleep::new(Duration::from_secs(3)).await;
    println!("progressing sleep {}", label);
    AsyncSleep::new(Duration::from_secs(2)).await;
    println!("done sleeping {}", label);
}


fn main() {
    let handle_one = spawn_task(sleeping(1));
    let handle_two = spawn_task(sleeping(2));
    let handle_three = spawn_task(sleeping(3));

    println!("before the sleep");
    std::thread::sleep(Duration::from_secs(5));
    println!("before the block");

    future::block_on(handle_one);
    future::block_on(handle_two);
    future::block_on(handle_three);
}

