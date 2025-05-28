/// A trait for items that can be removed from their container during iteration.
///
/// This trait is implemented by iterator items that allow in-place removal of elements
/// with O(1) time complexity by swapping the element to be removed with the last element
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
/// for item in numbers.removable_iter() {
///     if *item.get() % 2 == 0 {
///         // Remove even numbers
///         item.remove();
///     }
/// }
/// // Note: The order of remaining elements is not preserved
/// assert_eq!(numbers.len(), 3);
/// ```
pub trait RemovableItem<T> {
    /// Removes the current item from the container.
    ///
    /// This operation is O(1) as it swaps the current item with the last item in the container.
    /// The order of elements will not be preserved, unless the item is the last item in the
    /// container.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn remove(self);

    /// Returns a reference to the current item.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn get(&self) -> &T;
}

/// A trait for items that can be removed from their container during iteration, with mutable access.
///
/// This trait is implemented by iterator items that allow in-place removal of elements
/// with O(1) time complexity by swapping the element to be removed with the last element
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
/// for mut item in numbers.removable_iter_mut() {
///     if *item.get() % 2 == 0 {
///         // Remove even numbers
///         item.remove();
///     } else {
///         // Double odd numbers
///         *item.get_mut() *= 2;
///     }
/// }
/// // Note: The order of remaining elements is not preserved
/// assert_eq!(numbers.len(), 3);
/// assert_eq!(numbers, vec![2, 10, 6]);
/// ```
pub trait RemovableItemMut<T> {
    /// Removes the current item from the container.
    ///
    /// This operation is O(1) as it swaps the current item with the last item in the container.
    /// The order of elements will not be preserved, unless the item is the last item in the
    /// container.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn remove(self);

    /// Returns a reference to the current item.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn get(&self) -> &T;

    /// Returns a mutable reference to the current item.
    ///
    /// # Panics
    ///
    /// With the feature `loop-lifetime-guard` enabled, this will panic, if the item was
    /// moved outside the loops scope. Without the feature, this will cause undefined behavior.
    fn get_mut(&mut self) -> &mut T;
}   