## Cancellation by losing a `tokio::select!` race

### What this example embodies

`tokio::select!` polls several futures concurrently and returns as soon as the
**first** one completes. The moment a task wins, `select!` drops every other
branch's future (cancels it) right where it was suspended.

