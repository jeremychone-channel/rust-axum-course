Rust Axum Full Course source code. 

YouTube Full Course: https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q

MIT OR Apache, all free to use. 



## Important 1

On **Windows** for the "link" file issue when running the cargo test and run in parallel. The solution is to move `tests/quick_dev.rs` to the `examples` folder, rename the function to `#[tokio::main]`, and it should allow you to do the following:
- In Terminal 1: `cargo watch -q -c -w src/ -x run`
- In Terminal 2: `cargo watch -q -c -w examples/ -x 'run --example quick_dev'`

(we will follow this scheme in the next videos)

## Important 2

**Axum** static routes bug in `v0.6.15` has been fixed in `v0.6.16` make sure you do a `cargo update` if you had a previous git pull of this repo. 

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