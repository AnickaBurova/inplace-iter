


/// An iterator item which allows in place removal of the current item and returns it.
/// The iterator might cause reordering of the items in the container.
pub trait TakeableItem<T> {
    /// Take the current item and return it
    fn take(self) -> T;
    
    /// Get a reference to the current item, cannot be used after take.
    fn get(&self) -> &T;
}