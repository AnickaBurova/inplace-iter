//! Implementation of a removable iterator for a vector.
//!
use std::cell::RefCell;
use std::rc::Rc;
use crate::removable_iterator::RemovableItem;

/// An iterator which allows for the removal of items from the underlying vector.
/// The iterator will swap the current item with the last item, so the result will
/// cause unordered vector. Don't use when order is important.
pub struct RemovableVecIterator<'a, T> {
    /// A mutable reference to the vector from which items are taken.
    marked: &'a mut Vec<T>,
    /// A raw pointer to the vector data for unsafe access.
    data: *mut Vec<T>,
    /// A flag indicating whether an item has already been taken.
    removed: Rc<RefCell<bool>>,
    /// The current index in the vector, or None if iteration hasn't started.
    index: Option<usize>,
    /// The rotten indicator given to the last generated iterator item.
    #[cfg(feature = "loop-lifetime-guard")]
    last_rotten: Option<Rc<RefCell<bool>>>,

}

#[cfg(feature = "loop-lifetime-guard")]
impl<'a, T> Drop for RemovableVecIterator<'a, T> {
    fn drop(&mut self) {
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
    }
}

/// A struct representing an item that can be conditionally removed from the underlying vector.
pub struct RemovableVecItem<T> {
    /// A raw pointer to the vector containing the item.
    data: *mut Vec<T>,
    /// The index of the item within the vector.
    index: usize,
    /// A reference-counted cell indicating whether the item has been taken.
    removed: Rc<RefCell<bool>>,
    /// Indicator that this iterator item should no longer be used!
    #[cfg(feature = "loop-lifetime-guard")]
    rotten: Rc<RefCell<bool>>,
}

#[cfg(feature = "loop-lifetime-guard")]
impl<T> RemovableVecItem<T> {
    fn check_rotten(&self) {
        if *self.rotten.borrow() {
            panic!("This iterator item is no longer valid!");
        }
    }
}

impl<T> RemovableItem<T> for RemovableVecItem<T> {
    /// Remove the current item from the underlying vector.
    /// The last item is moved to this current place
    fn remove(self) {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        *self.removed.borrow_mut() = true;
        unsafe {
            let v = &mut (*self.data);
            let _ =if self.index == v.len() {
                // at the last item, no more items
                v.pop().unwrap()
            } else {
                v.swap_remove(self.index)
            };
        }
    }


    /// Get a reference to the current item from the underlying vector.
    /// Even after removal, this item is still valid and same, as the 
    /// actual removal happens on the next call to next.
    fn get(&self) -> &T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            let v = &mut (*self.data);
            &(*v.as_ptr().add(self.index))
        }
    }
}

impl<'a, T> RemovableVecIterator<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        let data = v as *mut Vec<T>;
        let marked = v;
        Self {
            marked,
            data,
            removed: Rc::new(RefCell::new(false)),
            index: None,
            #[cfg(feature = "loop-lifetime-guard")]
            last_rotten: None,
        }
    }
}

impl<'a, T> Iterator for RemovableVecIterator<'a, T> {
    type Item = RemovableVecItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "loop-lifetime-guard")]
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
        if self.marked.is_empty() {
            return None;
        }
        let index = if *self.removed.borrow() {
            *self.removed.borrow_mut() = false;
            self.index.unwrap() // if taken, then index is set and we don't increment to the next
        } else if let Some(index) = self.index {
            // move to the next item
            self.index = Some(index + 1);
            index + 1
        } else {
            // start at 0
            self.index = Some(0);
            0
        };
        if index < self.marked.len() {
            #[cfg(feature = "loop-lifetime-guard")]
            let rotten = {
                let rotten = Rc::new(RefCell::new(false));
                self.last_rotten = Some(rotten.clone());
                rotten
            };
            Some(RemovableVecItem {
                data: self.data,
                index,
                removed: self.removed.clone(),
                #[cfg(feature = "loop-lifetime-guard")]
                rotten,
            })
        } else {
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removable_iterator_basic() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = RemovableVecIterator::new(&mut a);
        assert_eq!(iter.next().unwrap().get(), &1);
        assert_eq!(iter.next().unwrap().get(), &2);
        let item = iter.next().unwrap();
        assert_eq!(item.get(), &3);
        assert_eq!(item.get(), &3);
        assert_eq!(iter.next().unwrap().get(), &4);
        assert_eq!(iter.next().unwrap().get(), &5);
        assert!(iter.next().is_none());
        // assert_eq!(a, vec![1, 2, 5, 4]);
        drop(iter);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_removable_iterator_remove_all() {
        let mut a = vec![1, 2, 3, 4, 5];
        for item in RemovableVecIterator::new(&mut a) {
            item.remove();
        }
        assert!(a.is_empty());
    }
    #[test]
    fn test_removable_iterator_remove_all_in_while() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = RemovableVecIterator::new(&mut a);
        while let Some(item) = iter.next() {
            item.remove();
        }
        drop(iter);
        assert!(a.is_empty());
    }

    #[test]
    fn test_removable_iterator_no_remove() {
        let mut a = vec![1, 2, 3, 4, 5];
        for iter in RemovableVecIterator::new(&mut a) {
            // do nothing   
            let _ = iter.get();
        }
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_removable_iterator_no_remove_outside_its_for() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = RemovableVecIterator::new(&mut a);
        for _ in 0..5 {
            iter.next();
        }
        drop(iter);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
}
