# Rust Collections Performance Checker

A simple command-line tool designed to provide a hands-on demonstration of the performance of various Data Structures/Collections, namely : (**Vec**<T>, **VecDeque**<T>, **LinkedList**<T>, **HashMap**<K, V>, **HashSet**<T>)

## How to Run

1. First of all you need to have the Rust toolchain installed. If you don't, you can get it from rustup.rs
2. Clone the project or save the files into a new Rust project directory (Perf_Checker) or just type "cargo new Perf_Checker" in the terminal and you will get the structure similar to the one below:

```bash
   Perf_Checker/
   ├── Cargo.toml
   └── src/
       └── main.rs
```
3. Copy Paste the Codes into main.rs and toml respectively and run "Cargo build" and your project is ready.
## Usage

```bash
# General syntax
cargo run --release -- [DATA_STRUCTURE] --size [NUMBER_OF_ELEMENTS]
```
Examples : 

Benchmark Vec with the default 1,000,000 elements:
```bash
cargo run --release -- vec
```

Benchmark LinkedList with 100,000 elements (and see how slow it is):
```bash
cargo run --release -- linked-list --size 100000

```

Thats it, Pay Close attention to the output for comparing.
