# Rust Axum Full Course source code

YouTube Full Course: https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q

MIT OR Apache, all free to use.

## Note 1

Moved the `tests/quick_dev.rs` as an `examples/quick_dev.rs` file (same code), as it is more fitting and seems to resolve a Windows limitation about running `test` and `run` simultaneously.

## Note 2

Make sure to use `axum` `0.6.16` or above, as `0.6.15` had a bug in the static routing. 

# Dev (REPL)

```sh
# Terminal 1 - For server run.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - For test.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

# Dev

```sh
# Terminal 1 - For server run.
cargo run

# Terminal 2 - For test.
cargo test quick_dev -- --nocapture
```

<br /><br /><br />
This repo: https://github.com/jeremychone-channel/rust-axum-course