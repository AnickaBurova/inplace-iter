//! In place iterators are modifying a container in place in the fastest possible way while giving 
//! you the convenience of a standard iterator.
//! It does rely on unsafe, so consider that before using.

mod removable_iterator;
mod removable_iterator_vec;

mod takeable_iterator;
mod takeable_iterator_vec;

mod inplace_vector;

pub mod prelude {
    pub use crate::removable_iterator::RemovableItem;
    pub use crate::takeable_iterator::TakeableItem;
    pub use crate::inplace_vector::InplaceVector;
}