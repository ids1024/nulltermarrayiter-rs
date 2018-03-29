#![cfg_attr(not(test), no_std)]

pub trait Zeroable {
    fn is_zero(self) -> bool;
}

macro_rules! impl_zeroable_num {
    ( $type:ty ) => {
        impl Zeroable for $type {
            fn is_zero(self) -> bool {
                self == 0
            }
        }
    }
}

impl_zeroable_num!(u8);
impl_zeroable_num!(i8);
impl_zeroable_num!(u16);
impl_zeroable_num!(i16);
impl_zeroable_num!(u32);
impl_zeroable_num!(i32);
impl_zeroable_num!(u64);
impl_zeroable_num!(i64);

impl<T> Zeroable for *const T {
    fn is_zero(self) -> bool {
        self.is_null()
    }
}

impl<T> Zeroable for *mut T {
    fn is_zero(self) -> bool {
        self.is_null()
    }
}

/// Iterator over a null-terminated array. Iteration stops when NULL or 0 is
/// reached.
pub struct NullTermArrayIter<T:  Zeroable + Copy> {
    ptr: *const T,
}

impl<T: Zeroable + Copy> NullTermArrayIter<T> {
    /// Create a new iterator from the raw pointer.
    /// 
    /// This results in undefined behavior if the pointer is invalid, or
    /// the array is not null-terminated.
    pub unsafe fn new(ptr: *const T) -> Self {
        Self { ptr }
    }
}

impl<T: Zeroable + Copy> Iterator for NullTermArrayIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let value = unsafe { *self.ptr };
        if value.is_zero() {
            None
        } else {
            self.ptr = unsafe { self.ptr.offset(1) };
            Some(value)
        }
    }
}

#[test]
fn test_nulltermarray_strings() {
    use std::ptr;

    let strlist = vec!["array\0", "of\0", "strings\0"];
    let mut ptrlist: Vec<_> = strlist.iter().map(|x| x.as_ptr()).collect();
    ptrlist.push(ptr::null());
    
    let mut iter = unsafe { NullTermArrayIter::new(ptrlist.as_ptr()) };
    for i in strlist.iter() {
        assert_eq!(iter.next(), Some(i.as_ptr()));
    }
    assert_eq!(iter.next(), None);
}

#[test]
fn test_nulltermarray_string() {
    let text = "string\0";
    let mut iter = unsafe { NullTermArrayIter::new(text.as_ptr()) };
    for i in text[..text.len()-1].as_bytes().iter() {
        assert_eq!(iter.next(), Some(*i));
    }
    assert_eq!(iter.next(), None);
}
