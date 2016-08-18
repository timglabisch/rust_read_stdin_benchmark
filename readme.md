# Read Stdin Performance Rust

Reading and parsing textfiles from stdin is very common.
A typically usecase would be analyzing / aggregating logfiles or writing map/reduce jobs.

# Data
the testdata is a csv'ish format.
it contains 13 ints and a string, example:
789,String,1,20,3,4,5,6,7,8,9,10,11,12
789,String,1,20,3,4,5,6,7,8,9,10,11,12

you can run cargo run --release producer to produce 3,6GB of data.

## Using the internal io buffer and nom for byte by byte parsing
the fastest way i found was using rusts internal io buffer and nom for parsing the data.

```
time cargo run --release --bin producer | cargo run --release --bin consumer
cargo run --release --bin producer  4.68s user 2.90s system 66% cpu 11.375 total
cargo run --release --bin consumer  9.00s user 0.64s system 84% cpu 11.375 total
```

Parsing 3.36GiB of data takes around *10 Seconds (342MiB/s)* on my 13" MacBook.

## Using Readline
normal times i (and every tutorial) is using `stdin.lock().lines()` to create an iterator for stdin. 

```
time cargo run --release --bin producer | cargo run --release --bin slow_consumer
cargo run --release --bin producer  4.64s user 2.78s system 22% cpu 33.046 total
cargo run --release --bin slow_consumer  32.10s user 0.75s system 99% cpu 33.046 total
```

Parsing 3.36GiB of data takes around *32 Seconds (106/s)* on my 13" MacBook.
