pub mod array;
pub mod matrix_array;
pub mod single_array;
pub mod vector_array;
pub mod list;

pub trait IArray<T> {
    fn size(&self) -> usize;
    fn push(&mut self, elem: T); // analog of void add(T item)
    fn get(&self, index: usize) -> &T;
    fn insert(&mut self, elem: T, index: usize); // analog of void add(T item, int index)
    fn remove(&mut self, index: usize) -> T;
}

#[derive(Debug, Default)]
pub struct Wrapper<T> {
    inner: Vec<T>,
}

impl<T> Wrapper<T> {
    pub fn new() -> Self {
        Self { inner: vec![] }
    }
}

impl<T> IArray<T> for Wrapper<T> {
    fn size(&self) -> usize {
        self.inner.len()
    }

    fn push(&mut self, elem: T) {
        self.inner.push(elem);
    }

    fn get(&self, index: usize) -> &T {
        self.inner.get(index).unwrap()
    }

    fn insert(&mut self, elem: T, index: usize) {
        self.inner.insert(index, elem);
    }

    fn remove(&mut self, index: usize) -> T {
        self.inner.remove(index)
    }
}