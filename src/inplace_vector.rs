use crate::inplace_vec_iterator::InplaceVecIterator;
use crate::removable_confirm_iterator_vec::{InplaceRemovableConfirmVecIterator, RemovableConfirmIterator};
use crate::removable_iterator::{RemovableItem, RemovableItemMut};
use crate::takeable_iterator::{TakeableItem, TakeableItemMut};

/// A trait that extends collections with methods for in-place iteration with removal/take operations.
///
/// This trait provides methods to create iterators that can modify the underlying collection
/// during iteration, specifically for removing or taking elements efficiently.
///
/// # Implementations
///
/// - `Vec<T>`: Standard library's vector implementation
///
/// # Examples
///
/// ## Using `removable_iter`
///
/// ```
/// use inplace_iter::prelude::*;
///
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// for item in numbers.removable_iter() {
///     if *item.get() % 2 == 0 {
///         item.remove();
///     }
/// }
/// assert_eq!(numbers.len(), 3);
/// ```
///
/// ## Using `takeable_iter`
///
/// ```
/// use inplace_iter::prelude::*;
///
/// let mut names = vec!["Alice".to_string(), "Bob".to_string(), "Charlie".to_string()];
/// let mut long_names = Vec::new();
///
/// for item in names.takeable_iter() {
///     if item.get().len() > 4 {
///         long_names.push(item.take());
///     }
/// }
///
/// assert_eq!(long_names, vec!["Alice", "Charlie"]);
/// assert_eq!(names.len(), 1);
/// assert_eq!(names[0], "Bob");
/// ```
pub trait InplaceVector<T> {
    /// Returns an iterator that allows taking ownership of elements during iteration.
    ///
    /// The iterator yields items that implement `TakeableItem<T>`, which provides
    /// a `take()` method to remove and return the current element.
    ///
    /// # Performance
    ///
    /// - Taking an element is O(1) time complexity
    /// - The order of elements is not preserved when taking elements
    fn takeable_iter(&mut self) -> impl Iterator<Item = impl TakeableItem<T>>;
    
    /// Returns an iterator that allows taking ownership of elements during iteration.
    ///
    /// The iterator yields items that implement `TakeableItem<T>`, which provides
    /// a `take()` method to remove and return the current element.
    ///
    /// # Performance
    ///
    /// - Taking an element is O(1) time complexity
    /// - The order of elements is not preserved when taking elements
    fn takeable_iter_mut(&mut self) -> impl Iterator<Item = impl TakeableItemMut<T>>;
    
    /// Returns an iterator that allows removing elements during iteration.
    ///
    /// The iterator yields items that implement `RemovableItem<T>`, which provides
    /// a `remove()` method to remove the current element.
    ///
    /// # Performance
    ///
    /// - Removal is O(1) time complexity
    /// - The order of elements is not preserved when removing elements
    fn removable_iter(&mut self) -> impl Iterator<Item = impl RemovableItem<T>>;

    /// Returns an iterator that allows removing elements during iteration.
    ///
    /// The iterator yields items that implement `RemovableItem<T>`, which provides
    /// a `remove()` method to remove the current element.
    ///
    /// # Performance
    ///
    /// - Removal is O(1) time complexity
    /// - The order of elements is not preserved when removing elements
    fn removable_iter_mut(&mut self) -> impl Iterator<Item = impl RemovableItemMut<T>>;

    /// Returns a wrapper around iterator that allows removing elements during iteration.
    /// The removals are not yet applied.
    /// The wrapper is then used to confirm or cancel the removals.
    ///
    /// The iterator yields items that implement `RemovableItem<T>`, which provides
    /// a `remove()` method to remove the current element.
    /// The wrapper provides `confirm_removals()` and `cancel_removals()` methods to apply or cancel the removals.
    ///
    /// # Performance
    ///
    /// - Removal is O(1) time complexity
    /// - The order of elements is not preserved when removing elements, even if the removals are cancelled.
    fn removable_confirm_iter(&mut self) -> impl RemovableConfirmIterator<Item = impl RemovableItem<T>>;
    
    /// Returns a wrapper around mutable iterator that allows removing elements during iteration.
    /// The removals are not yet applied.
    /// The wrapper is then used to confirm or cancel the removals.
    ///
    /// The iterator yields items that implement `RemovableItem<T>`, which provides
    /// a `remove()` method to remove the current element.
    /// The wrapper provides `confirm_removals()` and `cancel_removals()` methods to apply or cancel the removals.
    ///
    /// # Performance
    ///
    /// - Removal is O(1) time complexity
    /// - The order of elements is not preserved when removing elements, even if the removals are cancelled.
    fn removable_confirm_iter_mut(&mut self) -> impl RemovableConfirmIterator<Item = impl RemovableItemMut<T>>;
}

impl<T> InplaceVector<T> for Vec<T> {
    fn takeable_iter(&mut self) -> impl Iterator<Item = impl TakeableItem<T>> {
        InplaceVecIterator::new(self)
    }
    
    fn takeable_iter_mut(&mut self) -> impl Iterator<Item = impl TakeableItemMut<T>> {
        InplaceVecIterator::new(self)
    }
    
    fn removable_iter(&mut self) -> impl Iterator<Item = impl RemovableItem<T>> {
        InplaceVecIterator::new(self)
    }
    
    fn removable_iter_mut(&mut self) -> impl Iterator<Item = impl RemovableItemMut<T>> {
        InplaceVecIterator::new(self)
    }

    fn removable_confirm_iter(&mut self) -> impl RemovableConfirmIterator<Item=impl RemovableItem<T>> {
        InplaceRemovableConfirmVecIterator::new(self)
    }

    fn removable_confirm_iter_mut(&mut self) -> impl RemovableConfirmIterator<Item=impl RemovableItemMut<T>> {
        InplaceRemovableConfirmVecIterator::new(self)
    }
}