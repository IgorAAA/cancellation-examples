use std::time::Duration;
use tokio::time::sleep;

async fn timed_task(name: &str, millis: u64) -> String {
    println!("{name}: started, will finish in {millis}ms");
    sleep(Duration::from_millis(millis)).await;
    println!("{name}: finished");
    format!("{name}-result")
}

pub async fn run() {
    println!("\n--- select_all ---");

    // `select_all` needs `Unpin` futures, so box-and-pin each one.
    let futures = vec![
        Box::pin(timed_task("fast", 100)),
        Box::pin(timed_task("medium", 500)),
        Box::pin(timed_task("slow", 5000)),
    ];

    let (value, index, remaining) = futures::future::select_all(futures).await;
    println!("select_all: future #{index} won with: {value}");

    // Dropping `remaining` cancels the two futures that hadn't finished.
    println!("dropping {} remaining futures (cancelling them)", remaining.len());
    drop(remaining);
}
