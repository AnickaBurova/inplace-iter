//! # In-Place Iteration with Removal/Take Operations
//!
//! This library provides iterators that allow in-place modification of collections,
//! specifically optimized for scenarios where you need to remove or take elements
//! while iterating over a collection.
//! Removing or taking elements could change the order of elements in the collection.
//! Special iterator wrappers allows confirming/cancelling the removals.
//! Confirmation is only applicable to removable iterators, as the actual items are kept in the
//! collection until confirmation is received. Cannot be implemented for takeable iterators for the
//! obvious reasons.
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
//! 2. Enable the `loop-lifetime-guard` feature during development for additional safety checks
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
//! ### Removing elements while iterating with mutable access
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! for mut item in numbers.removable_iter_mut() {
//!     if *item.get() % 2 == 0 {
//!         // Remove even numbers
//!         item.remove();
//!     } else {
//!         // Double odd numbers
//!         *item.get_mut() *= 2;
//!     }
//! }
//! // Note: The order of remaining elements is not preserved
//! assert_eq!(numbers.len(), 3);
//! assert_eq!(numbers, vec![2, 10, 6]);
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
//! ### Taking elements while iterating with mutable access
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! let mut sum = 0;
//! for mut item in numbers.takeable_iter_mut() {
//!     if *item.get() > 3 {
//!         // Take ownership of elements > 3
//!         sum += item.take();
//!     } else {
//!         // Double odd numbers
//!         *item.get_mut() *= 2;
//!     }
//! }
//! assert_eq!(sum, 9); // 4 + 5
//! assert_eq!(numbers.len(), 3);
//! assert_eq!(numbers, vec![2, 4, 6]);
//! ```
//!
//! ### Removing elements while iterating with confirmation
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! let mut confirm = numbers.removable_confirm_iter();
//! for item in confirm.iter() {
//!     if *item.get() % 2 == 0 {
//!         item.remove(); // Efficiently remove even numbers
//!     }
//! }
//! // Multiple calls to `iter()` are allowed, and the subsequent iterations will not yield the removed elements.
//! let next_iter = confirm.iter().map(|i| *i.get()).collect::<Vec<_>>();
//! assert_eq!(next_iter, vec![1,5,3]);
//! confirm.confirm_removals();
//!
//!
//! assert_eq!(numbers, vec![1, 5, 3]);
//! ```
//!
//! ### Removing elements while iterating with confirmation, but cancelling
//! ```
//! use inplace_iter::prelude::*;
//!
//! let mut numbers = vec![1, 2, 3, 4, 5];
//! let mut confirm = numbers.removable_confirm_iter();
//! for item in confirm.iter() {
//!     if *item.get() % 2 == 0 {
//!         item.remove(); // Efficiently remove even numbers
//!     }
//! }
//! confirm.cancel_removals();
//!// Order after cancellation is not guaranteed. Also, if used on mutable iterator, the elements will stay
//!// modified after cancelling!
//! assert_eq!(numbers, vec![1, 5, 3, 4, 2]);
//! ```
//! ## Features
//!
//! - `loop-lifetime-guard`: Enables additional runtime checks to detect if the item is accessed outside
//!    the loop. It is enabled by default.

mod removable_iterator;
mod removable_iterator_vec;

mod removable_confirm_iterator_vec;

mod takeable_iterator;
mod takeable_iterator_vec;

pub mod inplace_vec_iterator;
mod inplace_vector;

pub mod prelude {
    pub use crate::removable_iterator::RemovableItem;
    pub use crate::removable_iterator::RemovableItemMut;
    pub use crate::takeable_iterator::TakeableItem;
    pub use crate::takeable_iterator::TakeableItemMut;
    pub use crate::inplace_vector::InplaceVector;
    pub use crate::removable_confirm_iterator_vec::RemovableConfirmIterator;
}