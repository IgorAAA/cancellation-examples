use std::time::Duration;
use tokio::time::sleep;

async fn long_task(name: &str) -> Result<String, String> {
    println!("{name}: started, working for 5s...");
    sleep(Duration::from_secs(5)).await;
    println!("{name}: finished"); // never printed below
    Ok(format!("{name}-ok"))
}

async fn failing_task(name: &str) -> Result<String, String> {
    println!("{name}: started, will fail in 100ms...");
    sleep(Duration::from_millis(100)).await;
    println!("{name}: failing now");
    Err(format!("{name} blew up"))
}

pub async fn run() {
    println!("\n--- try_join ---");
    let result = tokio::try_join!(
        long_task("task-A"),
        failing_task("task-B"),
        long_task("task-C"),
    );

    match result {
        Ok((a, b, c)) => println!("all succeeded: {a}, {b}, {c}"),
        Err(e) => println!(
            "try_join! returned early with error: {e:?}; task-A and task-C were cancelled"
        ),
    }
}