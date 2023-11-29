# Rust Axum Full Course Source Code

CODE UPDATED TO AXUM 0.7 [tag: axum-06-to-07](https://github.com/jeremychone-channel/rust-axum-course/releases/tag/axum-06-to-07)

This repository contains the source code for the Rust Axum Full Course, which is available on [YouTube](https://youtube.com/watch?v=XZtlD_m59sM&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q). The code is available under either the MIT or Apache license and is free to use.

Full Rust Web App Production Blueprint Course and Repository at https://rust10x.com/web-app (YouTube video: [Episode 01 - Rust Web App - Course to Production Coding](https://youtube.com/watch?v=3cA_mk4vdWY&list=PL7r-PXl6ZPcCIOFaL7nVHXZvBmHNhrh_Q))

Important: Make sure to refresh this repo. I implemented a fix on `cookie.add` for `AUTH_TOKEN` (see details below in the notes section).


Here is a [Per Chapter Fork](https://github.com/FloWi/rust-axum-course) by [@FloWi](https://github.com/FloWi). Big thanks!

## Dev (REPL)

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the tests.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo test quick_dev -- --nocapture
```

## Notes

- IMPORTANT - for `AUTH_TOKEN` cookie, make sure to call `cookie.set_path("/");`. See commit [- Fix AUTH_TOKEN cookie issue](https://github.com/jeremychone-channel/rust-axum-course/commit/0bcde6fd1e49e605e9352031538cceda9e4287eb)
- The `tests/quick_dev.rs` file has been moved to `examples/quick_dev.rs` (with the same code) as it is more appropriate and seems to resolve a Windows limitation when running `test` and `run` simultaneously.
- Use the `--poll` flag for cargo watch (latest 8.4.0) on my Fedora Linux environment - both server and test.
Things were not updating without it. [see #1](https://github.com/jeremychone-channel/rust-axum-course/issues/1)
- Make sure to use `axum` version `0.6.16` or above, as version `0.6.15` had a bug in the static routing.
- Here is a [Per Chapter Fork](https://github.com/FloWi/rust-axum-course) by [@FloWi](https://github.com/FloWi). Big thanks!

This repository can be found on [GitHub](https://github.com/jeremychone-channel/rust-axum-course).
