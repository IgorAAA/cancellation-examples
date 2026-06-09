//! * `timeout`    — cancel by timing out (`tokio::time::timeout`)
//! * `abort`      — cancel a spawned task explicitly (`JoinHandle::abort`)

mod timeout;

mod abort;

mod try_join;

#[tokio::main]
async fn main() {
    timeout::run().await;
    abort::run().await;
    try_join::run().await;
}
