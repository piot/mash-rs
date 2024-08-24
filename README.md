# Mash - Hashing Functions Library for Rust

This crate provides a collection of hashing functions for Rust, including [Murmur3](https://en.wikipedia.org/wiki/MurmurHash#MurmurHash3), a fast and efficient hash algorithm.

## Available Hash Functions

- **Murmur3 (32-bit)**: A fast, non-cryptographic hash function suitable for hash tables and other hash-based data structures.

## Installation

To use this crate in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
mash-rs = "0.0.1"
```

## Usage

```rust
use mash_rs::murmur3_32;

fn main() {
    let data = b"hello world";
    let seed = 42;  // Arbitrary seed value
    let hash = murmur3_32(data, seed);
    println!("Murmur3 hash value: {:#x}", hash);
}
```

## Description

The `murmur3_32` function computes a 32-bit hash value for the given data using the Murmur3 hash algorithm.

In addition to Murmur3, this library provides other hashing functions. Check the documentation for more details on available algorithms and how to use them.

## License

This is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
