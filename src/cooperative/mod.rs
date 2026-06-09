use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

async fn cooperative_work(token: CancellationToken) -> String {
    println!("cooperative_work: started");
    for step in 1..=5 {
        // `cancelled()` resolves as soon as the token is cancelled; the timer
        // represents one chunk of real work. Whichever happens first wins.
        tokio::select! {
            _ = token.cancelled() => {
                println!("cooperative_work: noticed cancellation, cleaning up gracefully");
                return String::from("cancelled-cleanly");
            }
            _ = sleep(Duration::from_millis(100)) => {
                println!("cooperative_work: finished step {step}/5");
            }
        }
    }
    println!("cooperative_work: finished all steps");
    String::from("done")
}

pub async fn run() {
    println!("\n--- token ---");
    let token = CancellationToken::new();

    let worker_token = token.clone(); // this clone can be canceled by cancelling `token` or directly
    let handle = tokio::spawn(cooperative_work(worker_token));

    // Let it make some progress, then signal cancellation explicitly.
    sleep(Duration::from_millis(250)).await;
    println!("main: cancelling the token");
    token.cancel(); // cancels the token and all its children as well!

    match handle.await {
        Ok(value) => println!("cooperative_work returned: {value}"),
        Err(e) => println!("cooperative_work join error: {e}"),
    }
}