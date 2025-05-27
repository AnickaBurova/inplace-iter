//! # In-Place Iteration with Removal/Take Operations
//!
//! This library provides iterators that allow in-place modification of collections,
//! specifically optimized for scenarios where you need to remove or take elements
//! while iterating over a collection.
//! Removing or taking elements could change the order of elements in the collection.
//!
//! ## Features
//!
//! - **Efficient Removal**: Remove elements in O(1) time by swapping with the last element
//! - **Zero-Copy Taking**: Take ownership of elements without cloning
//! - **Safe Abstraction**: Encapsulates unsafe code behind a safe API
//! - **Optional Lifetime Guard**: Additional runtime checks with `loop-lifetime-guard` feature
//!
//! ## Safety Considerations
//!
//! This library uses `unsafe` code to provide its functionality. While the API is designed to be safe,
//! incorrect usage can lead to undefined behavior. Always follow these guidelines:
//!
//! 1. Don't hold multiple mutable references to the same element
//! 2. Don't use an item after it has been removed/taken
//! 3. Enable the `loop-lifetime-guard` feature during development for additional safety checks
//!
//! ## Examples
//!
//! ### Removing elements while iterating
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! for item in numbers.removable_iter() {
//!     if *item.get() % 2 == 0 {
//!         item.remove(); // Efficiently remove even numbers
//!     }
//! }
//! assert_eq!(numbers, vec![1, 5, 3]);
//! ```
//!
//! ### Taking elements while iterating
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! for item in numbers.takeable_iter() {
//!     if *item.get() > 3 {
//!         sum += item.take(); // Take ownership of elements > 3
//!     }
//! }
//! assert_eq!(sum, 9); // 4 + 5
//! assert_eq!(numbers.len(), 3);
//! ```
//!
//! ## Features
//!
//! - `loop-lifetime-guard`: Enables additional runtime checks to detect if the item is accessed outside
//!    the loop. It is enabled by default.

mod removable_iterator;
mod removable_iterator_vec;

mod takeable_iterator;
mod takeable_iterator_vec;

pub mod inplace_vec_iterator;
mod inplace_vector;

pub mod prelude {
    pub use crate::removable_iterator::RemovableItem;
    pub use crate::takeable_iterator::TakeableItem;
    pub use crate::inplace_vector::InplaceVector;
}