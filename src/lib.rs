//! The `BareMetalDeque` represents a fixed-size double-ended queue analogous to [VecDeque](https://doc.rust-lang.org/std/collections/struct.VecDeque.html). It is implemented internally as a ring buffer.
//! 
//! Basic deque operations (push/pop front/back)
//! ```
//! use bare_metal_deque::BareMetalDeque;
//! 
//! let mut q = BareMetalDeque::<i64, 5>::new();
//! q.push_back(1);
//! assert_eq!(q.back().unwrap(), 1);
//! assert_eq!(q.front().unwrap(), 1);
//! 
//! q.push_back(2);
//! assert_eq!(q.back().unwrap(), 2);
//! assert_eq!(q.front().unwrap(), 1);
//! 
//! q.push_back(3);
//! assert_eq!(q.back().unwrap(), 3);
//! assert_eq!(q.front().unwrap(), 1);
//! 
//! assert_eq!(q.pop_front().unwrap(), 1);
//! assert_eq!(q.back().unwrap(), 3);
//! assert_eq!(q.front().unwrap(), 2);
//! 
//! q.push_front(4);
//! assert_eq!(q.back().unwrap(), 3);
//! assert_eq!(q.front().unwrap(), 4);
//! 
//! assert_eq!(q.pop_back().unwrap(), 3);
//! assert_eq!(q.back().unwrap(), 2);
//! assert_eq!(q.front().unwrap(), 4);
//! 
//! q.push_back(5);
//! assert_eq!(q.back().unwrap(), 5);
//! assert_eq!(q.front().unwrap(), 4);
//! 
//! // Indexing
//! assert_eq!(q[0], 4);
//! assert_eq!(q[1], 2);
//! assert_eq!(q[2], 5);
//! ```

#![cfg_attr(not(test), no_std)]

use core::{default::Default, ops::{Index, IndexMut}};

#[derive(Copy, Clone, Debug)]
pub struct BareMetalDeque<T: Default, const MAX_STORED: usize> {
    array: [T; MAX_STORED],
    start: usize,
    size: usize,
}

impl<T: Default, const MAX_STORED: usize> Index<usize> for BareMetalDeque<T, MAX_STORED> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[(self.start + index) % self.array.len()]
    }
}

impl<T: Default, const MAX_STORED: usize> IndexMut<usize> for BareMetalDeque<T, MAX_STORED> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.array[(self.start + index) % self.array.len()]
    }
}

impl<T: Copy + Clone + Default, const MAX_STORED: usize> Default for BareMetalDeque<T, MAX_STORED> {
    fn default() -> Self {
        Self { array: [T::default(); MAX_STORED], start: 0, size: 0 }
    }
}

impl <T: Copy + Clone + Default, const MAX_STORED: usize> BareMetalDeque<T, MAX_STORED> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        (0..self.len()).map(|i| &self[i])
    }

    pub fn push_front(&mut self, value: T) {
        if self.size == self.array.len() {
            panic!("Queue is full");
        }
        self.start = (if self.start == 0 {self.array.len()} else {self.start}) - 1;
        self.array[self.start] = value;
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        if self.size == self.array.len() {
            panic!("Queue is full");
        }
        let index = (self.start + self.size) % self.array.len();
        self.array[index] = value;
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let result = self.front();
        if result.is_some() {
            self.start = (self.start + 1) % self.array.len();
            self.size -= 1;
        }
        result
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let result = self.back();
        if result.is_some() {
            self.size -= 1;
        }
        result
    }

    pub fn front(&self) -> Option<T> {
        if self.size > 0 {
            Some(self.array[self.start])
        } else {
            None
        }        
    }

    pub fn back(&self) -> Option<T> {
        if self.size > 0 {
            let index = (self.start + self.size - 1) % self.array.len();
            Some(self.array[index])
        } else {
            None
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_test1() {
        const TEST_SIZE: usize = 10;

        let mut q: BareMetalDeque<usize, TEST_SIZE> = BareMetalDeque::new();
        assert!(q.is_empty());
        for i in 0..TEST_SIZE {
            q.push_back(i);
            assert_eq!(q.len(), i + 1);
        }
        assert!(!q.is_empty());

        for i in 0..TEST_SIZE {
            assert_eq!(i, q[i]);
        }

        for i in 0..TEST_SIZE {
            assert_eq!(q.len(), TEST_SIZE - i);
            assert_eq!(q.pop_front().unwrap(), i);
        }
        assert!(q.is_empty());

        for i in 0..TEST_SIZE {
            q.push_back(i);
        }
        for i in 0..TEST_SIZE / 2 {
            q.pop_front();
            q.push_back(i + TEST_SIZE);
        }
        for i in 0..q.len() {
            assert_eq!(i + TEST_SIZE / 2, q[i]);
        }
    }

    #[test]
    fn queue_test2() {
        let mut q = BareMetalDeque::<usize, 4>::new();
        assert!(q.is_empty());

        for x in 11..15 {
            q.push_back(x);
            assert!(!q.is_empty());
            assert_eq!(q.len(), x % 10);
            assert_eq!(q.front().unwrap(), 11);
        }

        for x in 11..15 {
            let old_len = q.len();
            let v = q.pop_front().unwrap();
            assert_eq!(x, v);
            assert_eq!(old_len - 1, q.len());
        }

        q.push_back(12);
        q.push_back(1);
        assert_eq!(q.pop_front().unwrap(), 12);
        for x in 2..5 {
            q.push_back(x);
        }
        for x in 1..5 {
            assert_eq!(x, q.pop_front().unwrap());
        }
    }

    #[test]
    fn front_stack_test() {
        let mut stack = BareMetalDeque::<usize, 4>::new();
        for x in 11..=14 {
            stack.push_front(x);
            assert!(!stack.is_empty());
            assert_eq!(stack.len(), x % 10);
            assert_eq!(stack.front().unwrap(), x);
        }

        for x in (11..=14).rev() {
            let old_len = stack.len();
            let v = stack.pop_front();
            assert_eq!(x, v.unwrap());
            assert_eq!(old_len - 1, stack.len());
        }

        stack.push_front(1);
        stack.push_front(12);
        assert_eq!(stack.pop_front().unwrap(), 12);
        for x in 2..5 {
            stack.push_front(x);
        }
        for x in (1..5).rev() {
            assert_eq!(x, stack.pop_front().unwrap());
        }
    }


    #[test]
    fn back_stack_test() {
        let mut stack = BareMetalDeque::<usize, 4>::new();
        for x in 11..=14 {
            stack.push_back(x);
            assert!(!stack.is_empty());
            assert_eq!(stack.len(), x % 10);
            assert_eq!(stack.back().unwrap(), x);
        }

        for x in (11..=14).rev() {
            let old_len = stack.len();
            let v = stack.pop_back().unwrap();
            assert_eq!(x, v);
            assert_eq!(old_len - 1, stack.len());
        }

        stack.push_back(1);
        stack.push_back(12);
        assert_eq!(stack.pop_back().unwrap(), 12);
        for x in 2..5 {
            stack.push_back(x);
        }
        for x in (1..5).rev() {
            assert_eq!(x, stack.pop_back().unwrap());
        }
    }
}
