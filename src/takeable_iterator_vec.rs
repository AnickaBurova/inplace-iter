use std::cell::RefCell;
use std::rc::Rc;
use crate::takeable_iterator::TakeableItem;

/// An iterator which allows you to take items from the underlying vector.
///
/// It is only valid to take an item if you have not already taken it.
pub struct TakeableVecIterator<'a, T> {
    /// A mutable reference to the vector from which items are taken.
    marked: &'a mut Vec<T>,
    /// A raw pointer to the vector data for unsafe access.
    data: *mut Vec<T>,
    /// A flag indicating whether an item has already been taken.
    taken: Rc<RefCell<bool>>,
    /// The current index in the vector, or None if iteration hasn't started.
    index: Option<usize>,
    /// The rotten indicator given to the last generated iterator item.
    #[cfg(feature = "loop-lifetime-guard")]
    last_rotten: Option<Rc<RefCell<bool>>>,
}

#[cfg(feature = "loop-lifetime-guard")]
impl<'a, T> Drop for TakeableVecIterator<'a, T> {
    fn drop(&mut self) {
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
    }
}

impl<'a, T> Iterator for TakeableVecIterator<'a, T> {
    type Item = TakeableVecItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "loop-lifetime-guard")]
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
        if self.marked.is_empty() {
            return None;
        }
        let index = if *self.taken.borrow() {
            *self.taken.borrow_mut() = false;
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
            Some(TakeableVecItem {
                data: self.data,
                index,
                taken: self.taken.clone(),
                #[cfg(feature = "loop-lifetime-guard")]
                rotten,
            })
        } else {
            None
        }
    }
}

impl<'a, T> TakeableVecIterator<'a, T> {
    /// Create a new `TakeableIterator` from a vector.
    ///
    /// # Example
    ///
    /// ```
    ///     use inplace_iter::prelude::InplaceVector;
    ///     use inplace_iter::prelude::TakeableItem;
    ///     let mut a = vec![1,2,3,4,5,6,7,8];
    ///     for (counter, mut i) in a.takeable_iter().enumerate() {
    ///         let v = i.get();
    ///         println!("[{counter}] v = {v}");
    ///         if v % 2 == 0 {
    ///             let v = i.take();
    ///             println!("taken = {v}");
    ///         }
    ///     }
    ///     println!("{a:?}");
    /// ```
    pub fn new(v: &'a mut Vec<T>) -> Self {
        let data = v as *mut Vec<T>;
        let marked = v;
        Self {
            marked,
            data,
            taken: Rc::new(RefCell::new(false)),
            index: None,
            #[cfg(feature = "loop-lifetime-guard")]
            last_rotten: None,
        }
    }
}

/// A struct representing an item that can be taken from the underlying vector.
pub struct TakeableVecItem<T> {
    /// A raw pointer to the vector containing the item.
    data: *mut Vec<T>,
    /// The index of the item within the vector.
    index: usize,
    /// A reference-counted cell indicating whether the item has been taken.
    taken: Rc<RefCell<bool>>,
    /// Indicator that this iterator item should no longer be used!
    #[cfg(feature = "loop-lifetime-guard")]
    rotten: Rc<RefCell<bool>>,
}

#[cfg(feature = "loop-lifetime-guard")]
impl<T> TakeableVecItem<T> {
    fn check_rotten(&self) {
        if *self.rotten.borrow() {
            panic!("This iterator item is no longer valid!");
        }
    }
}

impl<T> TakeableItem<T> for TakeableVecItem<T> {
    fn take(self) -> T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        *self.taken.borrow_mut() = true;
        unsafe {
            let v = &mut (*self.data);
            if self.index == v.len() {
                // at the last item, no more items
                v.pop().unwrap()
            } else {
                v.swap_remove(self.index)
            }
        }
    }

    fn get(&self) -> &T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            let v = &mut (*self.data);
            &(*v.as_ptr().add(self.index))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_takeable_iterator() {
        let mut a = vec![1,2,3,4,5,6,7,8];
        let mut iter = TakeableVecIterator::new(&mut a);
        assert_eq!(iter.next().unwrap().get(), &1);
        assert_eq!(iter.next().unwrap().get(), &2);
        assert_eq!(iter.next().unwrap().get(), &3);
        assert_eq!(iter.next().unwrap().get(), &4);
        assert_eq!(iter.next().unwrap().take(), 5);
        assert_eq!(iter.next().unwrap().get(), &8);
        assert_eq!(iter.next().unwrap().get(), &6);
        assert_eq!(iter.next().unwrap().take(), 7);
        assert!(iter.next().is_none());
        drop(iter);
        assert_eq!(a, vec![1,2,3,4,8,6]);
    }


    #[cfg(feature = "loop-lifetime-guard")]
    mod loop_lifetime_guard {
        use crate::takeable_iterator_vec::TakeableVecIterator;
        use crate::takeable_iterator::TakeableItem;

        #[test]
        #[should_panic]
        fn test_drop() {
            let mut a = vec![1,2,3,4,5,6,7,8];
            let mut save = None;
            for i in  TakeableVecIterator::new(&mut a) {
                if save.is_none() {
                    save = Some(i);
                }
            }
            if let Some(i) = save {
                assert_eq!(i.get(), &1);
            }
        }
    }


    #[test]
    fn test_empty() {
        let mut a: Vec<u32> = vec![];
        let mut iter = TakeableVecIterator::new(&mut a);
        assert!(iter.next().is_none());
    }
}
