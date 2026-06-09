use std::time::Duration;
use tokio::time::timeout;
use cancellation_examples::slow_work;

pub async fn run() {
    println!("\n--- timeout ---");
    let result = timeout(Duration::from_secs(1), slow_work()).await;

    match result {
        Ok(value) => println!("slow_work completed with: {value}"),
        Err(_) => println!("slow_work was cancelled (timed out before finishing)"),
    }

    // The "slow_work: finished" line never prints, proving the future was
    // cancelled mid-flight rather than running to completion.
}