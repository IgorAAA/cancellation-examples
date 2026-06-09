//! * `timeout`     — cancel by timing out (`tokio::time::timeout`)
//! * `abort`       — cancel a spawned task explicitly (`JoinHandle::abort`)
//! * `try_join`    — cancel siblings via `tokio::try_join!` short-circuit
//! * `select`      — cancel by losing a `tokio::select!` race
//! * `select_all`  — cancel the losers of `futures::future::select_all`
//! * `cooperative` — cooperative cancellation (`tokio_util::sync::CancellationToken`)

mod timeout;

mod abort;

mod try_join;

mod select;

mod select_all;

mod cooperative;

#[tokio::main]
async fn main() {
    timeout::run().await;
    abort::run().await;
    try_join::run().await;
    select::run().await;
    select_all::run().await;
    cooperative::run().await;
}
