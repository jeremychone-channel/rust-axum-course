Rust Axum Full Course source code. 

YouTube Full Course: https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q

MIT OR Apache, all free to use. 

> Important: Axum bug in v0.6.15 (set the version to `=0.6.12`). This bug only affects the ServeDir, but the impact is quite significant. The code has been updated to lock the version to v0.6.12 until the issue is resolved. https://github.com/tokio-rs/axum/issues/1931


# Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - For test.
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```

# Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For test.
cargo test quick_dev -- --nocapture
```