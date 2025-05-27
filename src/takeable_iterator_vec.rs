
#[cfg(test)]
mod tests {
    use crate::prelude::InplaceVector;
    use crate::prelude::TakeableItem;


    #[test]
    fn test_takeable_iterator() {
        let mut a = vec![1,2,3,4,5,6,7,8];
        let mut iter = a.takeable_iter();
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
        use crate::prelude::InplaceVector;
        use crate::prelude::TakeableItem;
        #[test]
        #[should_panic]
        fn test_drop() {
            let mut a = vec![1,2,3,4,5,6,7,8];
            let mut save = None;
            for i in  a.takeable_iter() {
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
        let mut iter = a.takeable_iter();
        assert!(iter.next().is_none());
    }
}
