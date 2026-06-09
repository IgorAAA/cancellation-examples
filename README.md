# cancellation-examples

A small project to show the cases how to cancel async tasks in Rust using [tokio](https://tokio.rs).

Each example lives in its own module directory under [`src/`](src/), containing
the runnable code (`mod.rs`) and a `README.md` explaining what it embodies. The
entry point [`src/main.rs`](src/main.rs) runs every example in sequence.

| Example                                   | Mechanism                               | Trigger                      | Style                             |
|-------------------------------------------|-----------------------------------------|------------------------------|-----------------------------------|
| [`timeout`](src/timeout/README.md)        | `tokio::time::timeout`                  | a deadline elapses           | forced drop                       |
| [`abort`](src/abort/README.md)            | `JoinHandle::abort()`                   | explicit call                | forced drop                       |
| [`try_join`](src/try_join/README.md)      | `tokio::try_join!`                      | a subtask returns `Err`      | forced drop                       |
| [`select`](src/select/README.md)          | `tokio::select!`                        | another branch wins the race | forced drop                       |
| [`select_all`](src/select_all/README.md)  | `futures::future::select_all`           | first future completes       | forced drop (developer's choose)  |
| [`cooperative`](src/cooperative/README.md) | `tokio_util::sync::CancellationToken`   | explicit `cancel()`          | **cooperative**                  |

