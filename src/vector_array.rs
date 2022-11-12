use std::{
    alloc::{self, Layout},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
};

use crate::IArray;

#[derive(Debug)]
pub struct VectorArray<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    vector: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for VectorArray<T> {
    fn default() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        VectorArray {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            vector: 5,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for VectorArray<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for VectorArray<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for VectorArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> VectorArray<T> {
    pub fn new(vector: usize) -> Self {
        Self {
            vector,
            ..Default::default()
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    fn grow(&mut self) {
        let new_cap = self.vector + self.cap;
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );
        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> IArray<T> for VectorArray<T> {
    fn size(&self) -> usize {
        self.len
    }

    fn push(&mut self, elem: T) {
        self.insert(elem, self.len);
    }

    fn get(&self, index: usize) -> &T {
        (self as &[T]).get(index).unwrap()
    }

    fn insert(&mut self, elem: T, index: usize) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap == self.len {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len += 1;
        }
    }

    fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bytes = VectorArray::<u8>::new(10);
        bytes.push(42);
        assert_eq!(42, bytes.pop().unwrap());
        assert!(bytes.pop().is_none());
    }

    #[test]
    fn bells_and_whistles() {
        let mut bytes = VectorArray::<u8>::new(10);
        (0..=u8::MAX).for_each(|x| bytes.push(x));
        assert_eq!(256, bytes.len());
        assert_eq!(22, bytes[22]);
        bytes[255] = 42;
        assert_eq!(&42, bytes.last().unwrap());
        *bytes.first_mut().unwrap() = 255;
        assert_eq!(*bytes.first().unwrap(), bytes.get(254) + 1);
    }

    #[test]
    fn array_interface() {
        let mut array = VectorArray::<i64>::new(10);
        array.insert(42, 0);
        array.insert(1024, 0);
        array.insert(-339, 2);
        array.insert(-851, 1);
        assert_eq!(&1024, array.get(0));
        assert_eq!(&(-851), array.get(1));
        assert_eq!(&42, array.get(2));
        assert_eq!(&(-339), array.get(3));
        assert_eq!(1024, array.remove(0));
        assert_eq!(&(-851), array.get(0));
        assert_eq!(&42, array.get(1));
        assert_eq!(&(-339), array.get(2));
    }
}
