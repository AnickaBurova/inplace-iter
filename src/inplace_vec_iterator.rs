#[cfg(feature = "loop-lifetime-guard")]
use std::cell::RefCell;
#[cfg(feature = "loop-lifetime-guard")]
use std::rc::Rc;
use crate::prelude::{RemovableItem, TakeableItem};
use crate::removable_iterator::RemovableItemMut;
use crate::takeable_iterator::TakeableItemMut;

/// An iterator which allows you to take items from the underlying vector.
///
/// It is only valid to take an item if you have not already taken it.
pub struct InplaceVecIterator<'a, T> {
    /// This tells the borrow checker that the underlying vector is borrowed and cannot be used otherwise.
    _lifetime_guard: &'a mut Vec<T>,
    /// A raw pointer to the vector data for unsafe access.
    data: *mut Vec<T>,
    /// A flag indicating whether an item has been removed.
    removed: bool,
    /// The current index in the vector, or None if iteration hasn't started.
    index: Option<usize>,
    /// The rotten indicator given to the last generated iterator item.
    #[cfg(feature = "loop-lifetime-guard")]
    last_rotten: Option<Rc<RefCell<bool>>>,
}

#[cfg(feature = "loop-lifetime-guard")]
impl<'a, T> Drop for InplaceVecIterator<'a, T> {
    fn drop(&mut self) {
        self.rotten_item();
    }
}

#[cfg(feature = "loop-lifetime-guard")]
impl<'a, T> InplaceVecIterator<'a, T> {
    fn rotten_item(&mut self) {
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
    }
}

impl<T> RemovableItem<T> for InplaceVecItem<T> {
    /// Remove the current item from the underlying vector.
    /// The last item is moved to this current place
    fn remove(self) {
        let _ = self.take_value();
    }


    /// Get a reference to the current item from the underlying vector.
    /// Even after removal, this item is still valid and same, as the 
    /// actual removal happens on the next call to next.
    fn get(&self) -> &T {
        self.get_value()
    }
}

impl<T> TakeableItem<T> for InplaceVecItem<T> {
    fn take(self) -> T {
        self.take_value()
    }

    fn get(&self) -> &T {
        self.get_value()
    }
}

impl<T> TakeableItemMut<T> for InplaceVecItem<T> {
    fn take(self) -> T {
        self.take_value()
    }

    fn get(&self) -> &T {
        self.get_value()
    }
    fn get_mut(&self) -> &mut T {
        self.get_value_mut()
    }
}

impl<T> RemovableItemMut<T> for InplaceVecItem<T> {
    fn remove(self) {
        let _ = self.take_value();
    }

    fn get(&self) -> &T {
        self.get_value()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_value_mut()
    }
}

impl<'a, T> Iterator for InplaceVecIterator<'a, T> {
    type Item = InplaceVecItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "loop-lifetime-guard")]
        self.rotten_item();
        let len = unsafe {
            let v = &mut (*self.data);
            if v.is_empty() {
                return None;
            }
            v.len()
        };
        let index = if self.removed {
            self.removed = false;
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
        if index < len {
            #[cfg(feature = "loop-lifetime-guard")]
            let rotten = {
                let rotten = Rc::new(RefCell::new(false));
                self.last_rotten = Some(rotten.clone());
                rotten
            };
            Some(InplaceVecItem::new(self.data, index, &mut self.removed, #[cfg(feature = "loop-lifetime-guard")] rotten))
        } else {
            None
        }
    }
}

// impl<'a, T> Iterator for InplaceVecIterator<'a, T> {
//     type Item = crate::takeable_iterator_vec::TakeableVecItem<T>;
//
//     fn next(&mut self) -> Option<Self::Item> {
// }

impl<'a, T> InplaceVecIterator<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        let data = v as *mut Vec<T>;
        Self {
            _lifetime_guard: v,
            data,
            removed: false,
            index: None,
            #[cfg(feature = "loop-lifetime-guard")]
            last_rotten: None,
        }
    }
}


/// A struct representing an item that can be taken from the underlying vector.
pub struct InplaceVecItem<T> {
    /// A raw pointer to the vector containing the item.
    data: *mut Vec<T>,
    /// The index of the item within the vector.
    index: usize,
    /// An indicator to the vector that we have removed the item
    removed: *mut bool,
    /// Indicator that this iterator item should no longer be used!
    #[cfg(feature = "loop-lifetime-guard")]
    rotten: Rc<RefCell<bool>>,
}

#[cfg(feature = "loop-lifetime-guard")]
impl<T> InplaceVecItem<T> {
    fn check_rotten(&self) {
        if *self.rotten.borrow() {
            panic!("This iterator item is no longer valid!");
        }
    }
}
impl<T> InplaceVecItem<T> {
    #[cfg(feature = "loop-lifetime-guard")]
    pub(crate) fn new(data: *mut Vec<T>, index: usize, removed: *mut bool, rotten: Rc<RefCell<bool>>) -> Self {
        Self {
            data,
            index,
            removed,
            rotten,
        }
    }
    #[cfg(not(feature = "loop-lifetime-guard"))]
    pub(crate) fn new(data: *mut Vec<T>, index: usize, removed: *mut bool) -> Self {
        Self {
            data,
            index,
            removed,
        }
    }
}

impl<T> InplaceVecItem<T> {
    pub(crate) fn take_value(self) -> T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            *self.removed = true;
            let v = &mut (*self.data);
            if self.index == v.len() {
                // at the last item, no more items
                v.pop().unwrap()
            } else {
                v.swap_remove(self.index)
            }
        }
    }

    pub(crate) fn get_value(&self) -> &T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            let v = &mut (*self.data);
            &(*v.as_ptr().add(self.index))
        }
    }

    pub(crate) fn get_value_mut(&self) -> &mut T {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            let v = &mut (*self.data);
            &mut (*v.as_mut_ptr().add(self.index))
        }
    }
}
