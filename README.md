# In-Place Iteration with Removal/Take Operations

[![Crates.io](https://img.shields.io/crates/v/inplace-iter)](https://crates.io/crates/inplace-iter)
[![Documentation](https://docs.rs/inplace-iter/badge.svg)](https://docs.rs/inplace-iter)
[![License](https://img.shields.io/crates/l/inplace-iter)](LICENSE-MIT)

A Rust library providing iterators that allow efficient in-place modification of collections
with O(1) removal and take operations.

## Features

- **Efficient Removal**: Remove elements in O(1) time by swapping with the last element
- **Zero-Copy Taking**: Take ownership of elements without cloning
- **Safe Abstraction**: Encapsulates unsafe code behind a safe API
- **Optional Lifetime Guard**: Additional runtime checks with `loop-lifetime-guard` feature (enabled by default)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
inplace-iter = "0.1"
```

## Examples

### Removing Elements

```rust
use inplace_iter::prelude::*;

let mut numbers = vec![1, 2, 3, 4, 5];
for item in numbers.removable_iter() {
    if *item.get() % 2 == 0 {
        item.remove(); // Efficiently remove even numbers
    }
}
// Order is not preserved, but all even numbers are removed
assert_eq!(numbers.len(), 3);
```

### Taking Elements

```rust
use inplace_iter::prelude::*;

let mut names = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
let mut long_names = Vec::new();

for item in names.takeable_iter() {
    if item.get().len() > 4 {
        long_names.push(item.take()); // Take ownership of long names
    }
}

assert_eq!(long_names, vec!["Alice".to_string(), "Charlie".to_string()]);
assert_eq!(names.len(), 1);
assert_eq!(names[0], "Bob");
```
### Taking Elements with Mutable Access
```rust
use inplace_iter::prelude::*;

let mut numbers = vec![1, 2, 3, 4, 5];
let mut sum = 0;
for mut item in numbers.takeable_iter_mut() {
    if *item.get() > 3 {
        // Take ownership of elements > 3
        sum += item.take();
    } else {
        // Double odd numbers
        *item.get_mut() *= 2;
    }
}
assert_eq!(sum, 9); // 4 + 5
assert_eq!(numbers.len(), 3);
assert_eq!(numbers, vec![2, 4, 6]);
```

### Removing elements while iterating with confirmation
```rust
use inplace_iter::prelude::*;

let mut numbers = vec![1, 2, 3, 4, 5];
let mut confirm = numbers.removable_confirm_iter();
for item in confirm.iter() {
    if *item.get() % 2 == 0 {
        item.remove(); // Efficiently remove even numbers
    }
}
// Multiple calls to `iter()` are allowed,
// and the subsequent iterations will not yield the removed elements.
let next_iter = confirm.iter().map(|i| *i.get()).collect::<Vec<_>>();
assert_eq!(next_iter, vec![1,5,3]);
confirm.confirm_removals();


assert_eq!(numbers, vec![1, 5, 3]);
```

### Removing elements while iterating with confirmation, but cancelling
```rust
use inplace_iter::prelude::*;

let mut numbers = vec![1, 2, 3, 4, 5];
let mut confirm = numbers.removable_confirm_iter();
for item in confirm.iter() {
    if *item.get() % 2 == 0 {
        item.remove(); // Efficiently remove even numbers
    }
}
confirm.cancel_removals();
// Order after cancellation is not guaranteed. 
// Also, if used on mutable iterator, the elements will stay
// modified after cancelling!
assert_eq!(numbers, vec![1, 5, 3, 4, 2]);
```
## Safety

This library uses `unsafe` code internally to provide its functionality. While the API is designed to be safe, incorrect usage can lead to undefined behavior. Always follow these guidelines:

1. Don't hold multiple mutable references to the same element
2. Don't use an item after it has been removed/taken
3. The `loop-lifetime-guard` feature (enabled by default) adds runtime checks to detect invalid item usage


# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-05-28
### Added
- Mutable versions of all iterators (`iter_mut()` variants)

## [0.3.0] - 2025-05-29
### Added
- Confirmation of removals iterator wrapper (mut and const)


## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
