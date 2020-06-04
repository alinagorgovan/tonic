# tonic_benchmarking
Benchmarking for tonic gRPC implementation

### Usage
Start both servers and wait for them to run:
```bash
cargo run --bin unix_server
cargo run --bin tcp_server
```

Start benchmarking:
```bash
cargo run --bin bench
```

### Results
Example:
```bash
running 6 tests
test message_size_100kb_tcp ... bench:   8,476,319 ns/iter (+/- 707,933)
test message_size_100kb_uds ... bench:   8,385,960 ns/iter (+/- 661,317)
test message_size_1kb_tcp   ... bench:   3,595,823 ns/iter (+/- 240,418)
test message_size_1kb_uds   ... bench:   3,484,396 ns/iter (+/- 404,160)
test message_size_1mb_tcp   ... bench:  44,349,913 ns/iter (+/- 2,259,088)
test message_size_1mb_uds   ... bench:  45,592,893 ns/iter (+/- 5,571,811)
```
