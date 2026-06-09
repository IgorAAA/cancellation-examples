# cancellation-examples

A small project to show the cases how to cancel async tasks in Rust using [tokio](https://tokio.rs).

Each example lives in its own module directory under [`src/`](src/), containing
the runnable code (`mod.rs`) and a `README.md` explaining what it embodies. The
entry point [`src/main.rs`](src/main.rs) runs every example in sequence.

| Example | Mechanism | Trigger | Style |
|---|---|---|---|
| [`timeout`](src/timeout/README.md) | `tokio::time::timeout` | a deadline elapses | forced drop |
| [`abort`](src/abort/README.md) | `JoinHandle::abort()` | explicit call | forced drop |
| [`try_join`](src/try_join/README.md) | `tokio::try_join!` | a sibling returns `Err` | forced drop |

## TBD
More cases / examples to be added