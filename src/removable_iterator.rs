/// An iterator item which allows in place removal of the current item
/// by switching it with the last item in the container. This allows
/// fast O(1) removal of the current item. The iterator will still continue
/// to the next item, but after the removal the next item will be the previously
/// last item.
/// After moving to the next item, this Item should not be used as it could yield
/// the wrong item or even wrong memory if it was the last item in the container and
/// the last item was removed.
pub trait RemovableItem<T> {
    /// Remove the current item from the container.
    fn remove(self);
    /// Get the reference to the current item.
    fn get(&self) -> &T;
}