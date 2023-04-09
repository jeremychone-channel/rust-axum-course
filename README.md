Rust Axum Full Course source code. 

YouTube Full Course: https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q

MIT OR Apache, all free to use. 

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