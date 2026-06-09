## Cancellation of the losers of `futures::future::select_all`

### What this example embodies

`futures::future::select_all` takes a collection of futures, polls them
concurrently, and resolves as soon as the **first** one completes.

Unlike `tokio::select!`, `select_all` does **not** drop the losers — it
hands them back. Cancellation is therefore the developer's explicit choice: dropping
`remaining` cancels those futures.