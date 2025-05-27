use crate::removable_iterator_vec::RemovableVecIterator;
use crate::takeable_iterator_vec::TakeableVecIterator;

pub trait InplaceVector<T> {
    fn takeable_iter(&mut self) -> TakeableVecIterator<T>;
    
    fn removable_iter(&mut self) -> RemovableVecIterator<T>;
}

impl<T> InplaceVector<T> for Vec<T> {
    fn takeable_iter(&mut self) -> TakeableVecIterator<T> {
        TakeableVecIterator::new(self)
    }
    fn removable_iter(&mut self) -> RemovableVecIterator<T> {
        RemovableVecIterator::new(self)
    }
}