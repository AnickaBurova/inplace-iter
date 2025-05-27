//! Traits and implementations for taking ownership of elements during iteration.

/// A trait for items that can be taken from their container during iteration.
///
/// This trait is implemented by iterator items that allow taking ownership of elements
/// with O(1) time complexity by swapping the element to be taken with the last element
/// in the container.
///
/// # Ordering
///
/// - The order of elements in the container is not preserved when taking elements. Only if
///   the item is the last item in the container as it is not swapped but simply popped.
///
/// # Examples
///
/// ```
/// use inplace_iter::prelude::*;
///
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// let mut sum = 0;
/// for item in numbers.takeable_iter() {
///     if *item.get() > 3 {
///         // Take ownership of elements > 3
///         sum += item.take();
///     }
/// }
/// assert_eq!(sum, 9); // 4 + 5
/// assert_eq!(numbers.len(), 3);
/// ```
pub trait TakeableItem<T> {
    /// Takes ownership of the current item, removing it from the container.
    ///
    /// This operation is O(1) as it swaps the current item with the last item in the container.
    /// The order of elements will not be preserved, unless the item is the last item.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn take(self) -> T;
    
    /// Returns a reference to the current item.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn get(&self) -> &T;
}