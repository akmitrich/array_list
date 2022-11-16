use crate::{array::Array, IArray};

#[derive(Default)]
pub struct PriorityQueue<T> {
    queue: Array<Priority<T>>,
}

impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: Array::new(),
        }
    }

    pub fn enqueue(&mut self, priority: i64, item: T) {
        let to_push = self.find_priority_or_insert_new(priority);
        to_push.stack.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let last = self.queue.last_mut()?;
        let result = last.stack.pop();
        if last.stack.is_empty() {
            self.queue.pop();
        }
        result
    }

    fn find_priority_or_insert_new(&mut self, p: i64) -> &mut Priority<T> {
        match self.find_priority_position(p) {
            Some(position) => &mut self.queue[position],
            None => self.insert_new_priority(p),
        }
    }

    fn find_priority_position(&self, p: i64) -> Option<usize> {
        self.queue.iter().position(|current| current.priority == p)
    }

    fn insert_new_priority(&mut self, p: i64) -> &mut Priority<T> {
        let pos = self.get_position_for_new_priority(p);
        self.queue.insert(Priority::new(p), pos);
        &mut self.queue[pos]
    }

    fn get_position_for_new_priority(&self, p: i64) -> usize {
        // TODO! refactor get_position to binary search
        let queue_len = self.queue.len();
        let mut pos = 0;
        while pos < queue_len && self.queue[pos].priority > p {
            pos += 1;
        }
        pos
    }
}

struct Priority<T> {
    pub priority: i64,
    pub stack: Stack<T>,
}

impl<T> Priority<T> {
    pub fn new(p: i64) -> Self {
        Self {
            priority: p,
            stack: Stack::new(),
        }
    }
}

struct Stack<T> {
    inner: Array<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            inner: Array::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    pub fn push(&mut self, elem: T) {
        self.inner.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut deque = PriorityQueue::<u8>::default();
        deque.enqueue(1, 1);
        deque.enqueue(0, 42);
        assert_eq!(42, deque.dequeue().unwrap());
        assert_eq!(1, deque.dequeue().unwrap());
        assert!(deque.dequeue().is_none());
    }

    #[test]
    fn priority() {
        let mut deque = PriorityQueue::<u32>::new();
        deque.enqueue(42, 1);
        deque.enqueue(42, 2);
        deque.enqueue(0, 3);
        deque.enqueue(-5, 4);
        deque.enqueue(3, 5);
        deque.enqueue(-5, 6);
        deque.enqueue(0, 7);
        let mut deq = vec![];
        while let Some(item) = deque.dequeue() {
            deq.push(item);
        }
        assert_eq!(vec![6, 4, 7, 3, 5, 2, 1], deq);
    }
}
