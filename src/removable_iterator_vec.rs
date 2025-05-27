#[cfg(test)]
mod tests {
    use crate::prelude::InplaceVector;
    use crate::prelude::RemovableItem;

    #[test]
    fn test_removable_iterator_basic() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = a.removable_iter();
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
        drop(item);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_removable_iterator_remove_all() {
        let mut a = vec![1, 2, 3, 4, 5];
        for item in a.removable_iter() {
            item.remove();
        }
        assert!(a.is_empty());
    }
    #[test]
    fn test_removable_iterator_remove_all_in_while() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = a.removable_iter();
        while let Some(item) = iter.next() {
            item.remove();
        }
        drop(iter);
        assert!(a.is_empty());
    }

    #[test]
    fn test_removable_iterator_no_remove() {
        let mut a = vec![1, 2, 3, 4, 5];
        for iter in a.removable_iter() {
            // do nothing   
            let _ = iter.get();
        }
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
    #[test]
    fn test_removable_iterator_no_remove_outside_its_for() {
        let mut a = vec![1, 2, 3, 4, 5];
        let mut iter = a.removable_iter();
        for _ in 0..5 {
            iter.next();
        }
        drop(iter);
        assert_eq!(a, vec![1, 2, 3, 4, 5]);
    }
}
