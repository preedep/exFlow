use std::time::Duration;
use clokwerk::{AsyncScheduler, Job, TimeUnits};



#[tokio::main]
async fn main(){

    // Create a new scheduler
    let mut scheduler = AsyncScheduler::new();
// Add some tasks to it
    scheduler
        .every(10.minutes())
        .plus(30.seconds())
        .run(|| async { println!("Simplest is just using an async block"); });

// Or spawn a task to run it forever
    tokio::spawn(async move {
        loop {
            scheduler.run_pending().await;
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    });
}