use std::time::Duration;
use tokio::time::sleep;
use cancellation_examples::slow_work;

pub async fn run() {
    println!("\n--- select ---");
    let (stop_tx, mut stop_rx) = tokio::sync::oneshot::channel::<()>();

    // Something elsewhere decides to stop the work and sends a signal.
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        println!("main: sending stop signal");
        let _ = stop_tx.send(());
    });

    tokio::select! {
        // `slow_work()` (5 seconds) races against a "stop" signal sent over a `oneshot`
        // channel after 50ms. The stop signal wins, so `slow_work()` is dropped
        value = slow_work() => println!("slow_work completed with: {value}"),
        _ = &mut stop_rx => println!("slow_work was cancelled (stop signal won the race)"),
    }
}