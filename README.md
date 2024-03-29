# Hackerrank Challenges in Rust

[Hackerrank](https://www.hackerrank.com/) challenges solved with the **Rust Programming Language**.

I am absolutely new to Rust, and I have decided to give it a shot by solving some challenges from Hackerrank. If you are interested and you want to correct or improve my code, please do! :slightly_smiling_face: I would love to see more experienced coders' approaches.

---

#### Instructions

To run (you need [Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/) installed):

```bash
#!/bin/bash
cargo run
```

You need to change the `main` function  with the challenge you would like to run:

```rust
// src/main.rs

mod challenges;
mod functions;
mod rust_basics;

fn main() {
    challenges::thirty_days_of_code::day_00::main();
}
```
