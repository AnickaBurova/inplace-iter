//! This will mark the items for removal, but only perform the removal on confirmation.

use std::cell::RefCell;
use std::rc::Rc;
use crate::prelude::RemovableItem;
use crate::removable_iterator::RemovableItemMut;

pub trait RemovableConfirmIterator {
    type Item;
    /// Create an iterator that iterates over the elements.
    /// Subsequent calls to this method will iterate over not yet removed elements.
    /// If you have modified the elements with mutable iterator, the subsequent calls will
    /// iterate over the modified elements.
    fn iter(&mut self) -> impl Iterator<Item = Self::Item>;
    /// Confirm removals of the elements marked for removal and return the container.
    fn confirm_removals(self);
    /// Cancel removals, but the order of the elements might not be preserved.
    /// If used on mutable iterator, the modified items will stay modified, no cancellation on
    /// the changes. Cancellation is only applicable to the size of the container!
    fn cancel_removals(self);
}

pub struct InplaceRemovableConfirmVecIterator<'a, T> {
    /// This tells the borrow checker that the underlying vector is borrowed and cannot be used otherwise.
    vector: &'a mut Vec<T>,
    /// A raw pointer to the vector data for unsafe access.
    data: *mut Vec<T>,
    /// A flag indicating whether an item has been marked for removal.
    removed: bool,
    /// The current index in the vector, or None if iteration hasn't started.
    index: Option<usize>,
    /// The current size after removals.
    size: usize,
    /// The rotten indicator given to the last generated iterator item.
    #[cfg(feature = "loop-lifetime-guard")]
    last_rotten: Option<Rc<RefCell<bool>>>,
}

impl<'a, T> RemovableConfirmIterator for InplaceRemovableConfirmVecIterator<'a, T> {
    type Item = InplaceRemovableConfirmVecItem<T>;
    
    fn iter(&mut self) -> impl Iterator<Item = Self::Item> {
        self.index = None; // reset iterator
        self
    }
    fn confirm_removals(self) {
        if self.size < self.vector.len() {
            self.vector.truncate(self.size);
        }
    }

    fn cancel_removals(self) {
        // do nothing
    }
}

impl<'a, T> InplaceRemovableConfirmVecIterator<'a, T> {
    pub fn new(v: &'a mut Vec<T>) -> Self {
        let data = v as *mut Vec<T>;
        Self {
            size: v.len(),
            vector: v,
            index: None,
            data,
            removed: false,
            #[cfg(feature = "loop-lifetime-guard")]
            last_rotten: None
        }
    }
}

trait BuildItem<T> {
    fn build_new(data: *mut Vec<T>, index: usize, size: *mut usize, removed: *mut bool, #[cfg(feature = "loop-lifetime-guard")] rotten: Rc<RefCell<bool>>) -> Self;
}


impl<'a, T> InplaceRemovableConfirmVecIterator<'a, T> {
    
    #[cfg(feature = "loop-lifetime-guard")]
    fn rotten_item(&mut self) {
        if let Some(rotten) = self.last_rotten.take() {
            *rotten.borrow_mut() = true;
        }
    }
    
    fn next_item<I: BuildItem<T>>(&mut self) -> Option<I> {
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
        if index >= self.size {
            // we have reached the end of the vector after removals
            return None;
        }
        if index < len {
            #[cfg(feature = "loop-lifetime-guard")]
            let rotten = {
                let rotten = Rc::new(RefCell::new(false));
                self.last_rotten = Some(rotten.clone());
                rotten
            };
            Some(I::build_new(self.data, index, &mut self.size, &mut self.removed, #[cfg(feature = "loop-lifetime-guard")] rotten))
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for InplaceRemovableConfirmVecIterator<'a, T> {
    type Item = InplaceRemovableConfirmVecItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item()
    }
}

pub struct InplaceRemovableConfirmVecItem<T> {
    /// A raw pointer to the vector containing the item.
    data: *mut Vec<T>,
    /// The index of the item within the vector.
    index: usize,
    /// An indicator to the vector that we have removed the item
    removed: *mut bool,
    /// The current size of the vector
    size: *mut usize,
    /// Indicator that this iterator item should no longer be used!
    #[cfg(feature = "loop-lifetime-guard")]
    rotten: Rc<RefCell<bool>>,
}

impl<T> BuildItem<T> for InplaceRemovableConfirmVecItem<T> {
    fn build_new(data: *mut Vec<T>, index: usize, size: *mut usize, removed: *mut bool,
                 #[cfg(feature = "loop-lifetime-guard")]
                 rotten: Rc<RefCell<bool>>) -> Self {
        Self {
            data,
            index,
            removed,
            size,
            #[cfg(feature = "loop-lifetime-guard")]
            rotten,
        }
    }
}

#[cfg(feature = "loop-lifetime-guard")]
impl<T> InplaceRemovableConfirmVecItem<T> {
    fn check_rotten(&self) {
        if *self.rotten.borrow() {
            panic!("This iterator item is no longer valid!");
        }
    }
}
impl<T> InplaceRemovableConfirmVecItem<T> {
    pub(crate) fn remove_value(self) {
        #[cfg(feature = "loop-lifetime-guard")]
        self.check_rotten();
        unsafe {
            *self.removed = true;
            *self.size -= 1;
            let v = &mut (*self.data);
            if self.index < *self.size {
                // swap with the last item, but our last item
                v.swap(self.index, *self.size);
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

impl<T> RemovableItem<T> for InplaceRemovableConfirmVecItem<T> {
    fn remove(self) {
        self.remove_value();
    }

    fn get(&self) -> &T {
        self.get_value()
    }
}

impl<T> RemovableItemMut<T> for InplaceRemovableConfirmVecItem<T> {
    fn remove(self) {
        self.remove_value();
    }

    fn get(&self) -> &T {
        self.get_value()
    }

    fn get_mut(&mut self) -> &mut T {
        self.get_value_mut()
    }
}

