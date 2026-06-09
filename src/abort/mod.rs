use std::time::Duration;
use tokio::time::sleep;
use cancellation_examples::slow_work;

pub async fn run() {
    println!("\n--- abort ---");
    let handle = tokio::spawn(slow_work());

    // Give the task a moment to actually start running its body.
    sleep(Duration::from_millis(50)).await;

    println!("main: deciding to cancel slow_work now");
    handle.abort();

    // Awaiting an aborted task yields a `JoinError` for which `is_cancelled()`
    // is true.
    match handle.await {
        Ok(value) => println!("slow_work completed with: {value}"),
        Err(e) if e.is_cancelled() => println!("slow_work was cancelled (aborted)"),
        Err(e) => println!("slow_work failed: {e}"),
    }
}