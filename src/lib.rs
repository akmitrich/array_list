pub mod array;
pub mod matrix_array;
pub mod single_array;
pub mod vector_array;

pub trait IArray<T> {
    fn size(&self) -> usize;
    fn push(&mut self, elem: T); // analog of void add(T item)
    fn get(&self, index: usize) -> &T;
    fn insert(&mut self, elem: T, index: usize); // analog of void add(T item, int index)
    fn remove(&mut self, index: usize) -> T;
}
