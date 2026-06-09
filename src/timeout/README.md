## Cancellation by timeout

### What this example embodies

This is the most fundamental form of cancellation in async Rust: **dropping a
future cancels it.** `tokio::time::timeout` wraps an inner future and polls it
for at most a given duration. If the inner future hasn't completed by the time
the deadline elapses, `timeout` simply stops polling it and drops it. The
dropped future is abandoned at its last `.await` point and never makes progress
again.