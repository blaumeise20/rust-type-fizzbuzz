# Type level FizzBuzz in Rust

Rust's trait system is so powerful that you can use it to implement FizzBuzz at the type level. I got inspired by [this talk](https://rustlab.it/talks/rust-s-trait-system-is-a-proof-engine-let-s-make-it-prove-us-an-abi) at RustLab 2023.

## How to run

```bash
cargo run
```
It will print out a debug-formatted array of 100 elements which have been fully computed at compile time using no `const fn`s.
