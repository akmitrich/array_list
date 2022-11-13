use std::fmt::Debug;

use crate::{array::Array, vector_array::VectorArray, IArray};

#[derive(Debug)]
pub struct MatrixArray<T: Debug> {
    inner: Array<VectorArray<T>>,
    size: usize,
    vector: usize,
}

impl<T: Debug> Default for MatrixArray<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            size: 0,
            vector: 5, 
        }
    }
}

impl<T: Debug> IArray<T> for MatrixArray<T> {
    fn size(&self) -> usize {
        self.size
    }

    fn push(&mut self, elem: T) {
        self.insert(elem, self.size);
    }

    fn get(&self, index: usize) -> &T {
        let (inner_index, single_index) = self.make_indice(index);
        let store = self.inner.get(inner_index);
        store.get(single_index)
    }

    fn insert(&mut self, elem: T, index: usize) {
        self.make_room();
        let (inner_index, single_index) = self.make_indice(index);
        let store = self.inner.get_mut(inner_index).unwrap();
        store.insert(elem, single_index);
        self.size += 1;
        if store.size() > self.vector {
            let last_elem = store.remove(store.size() - 1);
            self.insert(last_elem, (inner_index + 1) * self.vector);
        }
    }

    fn remove(&mut self, index: usize) -> T {
        let (inner_index, single_index) = self.make_indice(index);
        let store = self.inner.get_mut(inner_index).unwrap();
        let removed = store.remove(single_index);
        self.shift_after_remove(inner_index);
        self.check_last_and_remove_if_empty();
        self.size -= 1;
        removed
    }
}

impl<T: Debug> MatrixArray<T> {
    pub fn new(vector: usize) -> Self {
        Self { vector, ..Default::default() }
    }

    pub fn repr(&self) -> String {
        let mut result: Vec<String> = vec![String::from("[")];
        for i in 0..self.inner.size() {
            result.push(format!("{:?},", self.inner.get(i) as &[T]));
        }
        result.push(String::from("]"));
        result.join("\n")
    }

    fn make_indice(&self, index: usize) -> (usize, usize) {
        (index / self.vector, index % self.vector)
    }

    fn make_room(&mut self) {
        if self.need_to_grow() {
            self.inner.push(VectorArray::new(self.vector));
        }
    }

    fn need_to_grow(&self) -> bool {
        if let Some(last) = self.inner.last() {
            last.size() >= self.vector
        } else {
            true
        }
    }

    fn check_last_and_remove_if_empty(&mut self) {
        if let Some(last) = self.inner.last() {
            if last.is_empty() {
                let last_index = self.inner.size() - 1;
                self.inner.remove(last_index);
            }
        }
    }

    fn shift_after_remove(&mut self, inner_index: usize) {
        for inner_index in (inner_index + 1)..self.inner.size() {
            let shifted = self.inner[inner_index].remove(0);
            let prev = self.inner.get_mut(inner_index - 1).unwrap();
            prev.push(shifted);
        }   
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let mut bytes = MatrixArray::<u8>::new(100);
        assert_eq!(0, bytes.size());
        bytes.push(42);
        assert_eq!(1, bytes.size());
    }

    #[test]
    fn push_get_insert_remove() {
        let mut array = MatrixArray::<i64>::new(10);
        for i in -42..42 {
            array.push(i);
        }
        assert_eq!(84, array.size());
        assert_eq!(&0, array.get(42));
        array.insert(686, 42);
        assert_eq!(&686, array.get(42));
        assert_eq!(&7, array.get(50));
        array.remove(42);
        assert_eq!(&0, array.get(42));
        for _ in 0..13 {
            array.remove(42);
        }
        println!("{}", array.repr());
    }

    #[test]
    fn insert() {
        let mut bytes = MatrixArray::<u8>::new(3);
        for i in 0..10 {
            bytes.push(i);
        }
        bytes.insert(42, 4);
        assert_eq!(0, bytes.remove(0));
        assert_eq!(6, bytes.remove(6));
        println!("{}", bytes.repr());
    }
}