# Read Stdin Performance Rust

time cargo run --release --bin producer | cargo run --release --bin slow_consumer
cargo run --release --bin producer  4.64s user 2.78s system 22% cpu 33.046 total
cargo run --release --bin slow_consumer  32.10s user 0.75s system 99% cpu 33.046 total
3.36GiB 0:00:32 [ 106MiB/s]

time cargo run --release --bin producer | cargo run --release --bin consumer
cargo run --release --bin producer  4.68s user 2.90s system 66% cpu 11.375 total
cargo run --release --bin consumer  9.00s user 0.64s system 84% cpu 11.375 total
3.36GiB 0:00:10 [ 342MiB/s]
