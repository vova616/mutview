#![feature(const_generics,test)]

use std::cell::{Cell, UnsafeCell};
use core::mem::MaybeUninit;

pub struct RefCells<'a, V, const N: usize> {
    pub len: Cell<usize>,
    keys: [Cell<MaybeUninit<usize>>; N],
    view: &'a [UnsafeCell<V>],
}

impl<'a, V, const N: usize> RefCells<'a, V, {N}> {
    pub fn new(slice: &'a mut [V]) -> Self {
        let keys: [Cell<MaybeUninit<usize>>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };
        RefCells {
            view: unsafe { std::mem::transmute(slice) },
            keys: keys,
            len: Cell::new(0),
        }
    }

    pub fn get(&self, key: usize) -> Option<&V> {
        self.get_mut(key).map(|v| &*v)
    }

    pub fn get_mut(&self, key: usize) -> Option<&mut V> {
        let len = self.len.get();
        if self.keys[..len]
            .iter()
            .find(|&i| {
                let i = unsafe {
                    i.get().assume_init()
                }; 
                i == key })
            .is_some()
        {
            panic!("borrowing more than once is not allowed")
        }
        
        if len >= N {
            panic!("reached max borrows, you can increase the size of N if needed")
        }
        self.keys[len].set(MaybeUninit::new(key));
        self.len.set(len + 1);
        
        let item = self.view.get(key)?;
        
        unsafe {
            Some(&mut *item.get())
        }
    }
}


extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn numbers_test() {
        let mut arr = [10,20,30,40,50];
        let view = RefCells::<_, 3>::new(&mut arr);
        let a = view.get_mut(0).unwrap();
        let b = view.get_mut(1).unwrap();
        let c = view.get_mut(2).unwrap();

        *a += *b + *c;
        *b += 2;
        *c += 3;

        let expected = [60,22,33,40,50];
        assert_eq!(arr, expected, "\nExpected\n{:?}\nfound\n{:?}", expected, arr);
    }

    #[test]
    fn strings_test() {
        let mut arr = ["", "a", "b"];
        let view = RefCells::<_, 1>::new(&mut arr);
        let a = "testtesttest".to_string();
        *view.get_mut(0).unwrap() = a.as_str();

        let expected = ["testtesttest", "a", "b"];
        assert_eq!(arr, expected, "\nExpected\n{:?}\nfound\n{:?}", expected, arr);
    }

    #[test]
    #[should_panic]
    fn strings_zero_test() {
        let mut arr = ["", "a", "b"];
        let view = RefCells::<_, 0>::new(&mut arr);
        let a = "testtesttest".to_string();
        *view.get_mut(0).unwrap() = a.as_str();

        let expected = ["testtesttest", "a", "b"];
        assert_eq!(arr, expected, "\nExpected\n{:?}\nfound\n{:?}", expected, arr);
    }

    #[test]
    #[should_panic]
    fn numbers_test_panic() {
        let mut arr = [10,20,30,40,50];
        let view = RefCells::<_, 2>::new(&mut arr);
        let a = view.get_mut(0).unwrap();
        let b = view.get_mut(1).unwrap();
        let c = view.get_mut(2).unwrap();
    }

    #[test]
    #[should_panic]
    fn numbers_test_panic2() {
        let mut arr = [10,20,30,40,50];
        let view = RefCells::<_, 2>::new(&mut arr);
        let a = view.get_mut(1).unwrap();
        let b = view.get_mut(1).unwrap();
    }
}
