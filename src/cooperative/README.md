## Cooperative cancellation with `CancellationToken`

### What this example embodies

`tokio_util::sync::CancellationToken` provides **cooperative** cancellation,
which is fundamentally different from every other example in this project. The
others all cancel by *forcibly dropping* a future at an `.await` point — the work
has no say and no chance to clean up.

With a token, the work **observes** cancellation and decides how to wind down.

A token can be **cloned and shared**, so a single `cancel()` call can stop many
tasks at once.