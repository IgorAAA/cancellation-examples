## Cancellation via `tokio::try_join!` short-circuit

`tokio::try_join!` drives several futures concurrently and waits for **all** of
them to succeed. But the instant any one returns `Err`, it short-circuits: it
stops polling the remaining futures and drops them.