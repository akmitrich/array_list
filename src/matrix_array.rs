use crate::{array::Array, vector_array::VectorArray, IArray};

pub struct MatrixArray<T> {
    inner: Array<VectorArray<T>>,
    vector: usize,
}

impl<T> Default for MatrixArray<T> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            vector: 5, 
        }
    }
}

impl<T> IArray<T> for MatrixArray<T> {
    fn size(&self) -> usize {
        self.inner
            .iter()
            .fold(0, |sum, inner_array| sum + inner_array.size())
    }

    fn push(&mut self, elem: T) {
        self.insert(elem, self.size());
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
    }

    fn remove(&mut self, index: usize) -> T {
        let (inner_index, single_index) = self.make_indice(index);
        let store = self.inner.get_mut(inner_index).unwrap();
        store.remove(single_index)
    }
}

impl<T> MatrixArray<T> {
    pub fn new(vector: usize) -> Self {
        Self { vector, ..Default::default() }
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
            last.size() == self.vector
        } else { // inner array is empty
            true
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
        let mut array = MatrixArray::<i64>::new(1);
        for i in -42..42 {
            array.push(i);
        }
        assert_eq!(84, array.size());
        assert_eq!(&0, array.get(42));
        array.insert(686, 42);
        assert_eq!(&686, array.get(42));
        array.remove(42);
        assert_eq!(&0, array.get(42));
    }
}