use std::time::Duration;
use tokio::time::sleep;

pub async fn slow_work() -> String {
    println!("slow_work: started, sleeping for 5s...");
    sleep(Duration::from_secs(5)).await;
    // By design, we should never reach this line, because the task is aborted before the sleep
    // finishes.
    println!("slow_work: finished");
    String::from("done")
}