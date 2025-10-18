# yes-uring

Feel like `yes` spends too much time in syscalls?

`yes-uring` reimplements `yes` with `io-uring`, with zero syscall overhead.

This means that `yes-uring` can spend 100% of its time in userspace!

## Requirements

This requires Linux kernel 5.13 or later (5.11 or later without root permissions) for `io_uring` support.

## Building

```bash
cargo build --release
```

The binary is output to `target/release/yes-uring`.

## Usage

```
Repeatedly output a line with all specified STRING(s), or 'y'

Usage: yes-uring [OPTIONS] [STRINGS]...

Arguments:
  [STRINGS]...  The string(s) to output repeatedly (default: "y")

Options:
      --ring-capacity <RING_CAPACITY>  Ring buffer capacity [default: 8]
      --sqpoll [<SQPOLL>]              Enable SQPOLL mode with an optionally specified idle time
      --threads <THREADS>              Number of threads to use [default: 1]
      --cpu-threads                    Use number of CPU cores as thread count
  -h, --help                           Print help
  -V, --version                        Print version
```