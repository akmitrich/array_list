use crate::{array::Array, IArray};

#[derive(Default)]
pub struct SparseArray<T: Default> {
    inner: Array<Value<T>>,
    zero: T,
    len: usize,
}

impl<T: Default + PartialEq> SparseArray<T> {
    pub fn new() -> Self {
        Self {
            inner: Array::new(),
            ..Default::default()
        }
    }

    fn find_inner_value(&self, index: usize) -> Option<&Value<T>> {
        self.inner.iter().find(|current| current.index == index)
    }

    fn inner_insert(&mut self, elem: T, index: usize) {
        let mut pos = 0;
        while pos < self.inner.len() && index > self.inner[pos].index {
            pos += 1;
        }
        for v in self.inner.iter_mut().skip(pos) {
            v.index += 1;
        }
        self.len += 1;
        if self.zero != elem {
            self.inner.insert(Value::new(index, elem), pos);
        }
    }

    pub fn repr(&self) -> String
    where
        T: std::fmt::Display,
    {
        let mut result = vec![];
        for v in self.inner.iter() {
            result.push(format!("({}, {})", v.index, v.value));
        }
        format!("[{}]", result.join(", "))
    }
}

impl<T: std::fmt::Display + Default + PartialEq> std::fmt::Display for SparseArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = vec![];
        for i in 0..self.len {
            result.push(format!("{}", self.get(i)));
        }
        write!(f, "[{}]", result.join(", "))
    }
}

impl<T: Default + PartialEq> IArray<T> for SparseArray<T> {
    fn size(&self) -> usize {
        self.len
    }

    fn push(&mut self, elem: T) {
        self.insert(elem, self.len);
    }

    fn get(&self, index: usize) -> &T {
        match self.find_inner_value(index) {
            Some(found) => &found.value,
            None => &self.zero,
        }
    }

    fn insert(&mut self, elem: T, index: usize) {
        if index < self.len {
            self.inner_insert(elem, index);
        } else {
            self.len = index + 1;
            if self.zero != elem {
                self.inner.push(Value::new(index, elem));
            }
        }
    }

    fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        self.len -= 1;
        match self.find_inner_value(index) {
            Some(_) => {
                let pos = self
                    .inner
                    .iter()
                    .position(|current| current.index == index)
                    .unwrap();
                let to_return = self.inner.remove(pos).value;
                for v in self.inner.iter_mut().skip(pos) {
                    v.index -= 1;
                }
                to_return
            }
            None => {
                for v in self.inner.iter_mut() {
                    if v.index > index {
                        v.index -= 1;
                    }
                }
                Default::default()
            }
        }
    }
}

#[derive(Default)]
struct Value<T> {
    pub value: T,
    pub index: usize,
}

impl<T> Value<T> {
    pub fn new(i: usize, v: T) -> Self {
        Self { value: v, index: i }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = SparseArray::<u8>::new();
        a.insert(0, 42);
        assert_eq!(43, a.size());
        a.insert(1, 28); // index of 0 becomes 43
        assert_eq!(44, a.size());
        a.insert(2, 43); // index of 0 becomes 44
        assert_eq!(45, a.size());
        a.insert(3, 42); // index of 2 becomes 44, index of 0 becomes 45
        a.insert(4, 105);
        assert_eq!(106, a.size());
        assert_eq!(&0, a.get(23));
        assert_eq!(&1, a.get(28));
        assert_eq!(&2, a.get(44));
        assert_eq!(&3, a.get(42));
        assert_eq!(&0, a.get(45));
        a.remove(55); // index of 4 becomes 104
        a.remove(33); // index of 2 becomes 43, index of 3 becomes 41, index of 4 becomes 103
        a.remove(42); // index of 2 becomes 42, index of 4 becomes 102
        a.remove(28); // index of 2 becomes 41, index of 3 becomes 40, index of 4 becomes 101
        assert_eq!(&2, a.get(41));
        assert_eq!(&3, a.get(40));
        assert_eq!(&4, a.get(101));
        a.insert(0, 105);
        println!("{}", a.repr());
        println!("{}", a);
    }
}
