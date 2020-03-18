// TODO: Remove me
#![allow(dead_code)]

use std::{collections::VecDeque, fmt};

/// Counter queue is basically a queue which
/// tracks the amount of the processed elements.
#[derive(Debug)]
pub struct CounterQueue<T: fmt::Debug> {
    elements: VecDeque<T>,
    counter: usize,
}

impl<T: fmt::Debug> Default for CounterQueue<T> {
    fn default() -> Self {
        Self {
            counter: 0,
            elements: VecDeque::new(),
        }
    }
}

impl<T: fmt::Debug> CounterQueue<T> {
    /// Creates a new empty counter queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty counter queue with the custom number of processed elements.
    /// This method is used to restore the state of the queue.
    pub fn new_with_count(counter: usize) -> Self {
        Self {
            counter,
            ..Default::default()
        }
    }

    /// Inserts an element to the end of the queue.
    pub fn push_back(&mut self, element: T) {
        self.elements.push_back(element);
    }

    /// Attempts to take the next element from the queue. Returns `None`
    /// if the queue is empty.
    ///
    /// Taking the actual value updates the counter.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.elements.pop_front() {
            Some(element) => {
                self.counter += 1;
                Some(element)
            }
            None => None,
        }
    }

    /// Returns the value of the counter.
    pub fn get_count(&self) -> usize {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the main operations of the queue: `push_back`, `pop_front` and `get_count`.
    #[test]
    fn basic_operations() {
        let mut queue: CounterQueue<String> = CounterQueue::new();

        // Check that by default the current count is 0.
        assert_eq!(queue.get_count(), 0);

        // Insert the next element and obtain it.
        queue.push_back("one".into());
        // Inserting the element should NOT update the counter.
        assert_eq!(queue.get_count(), 0);
        assert_eq!(queue.pop_front().unwrap(), "one");
        // After taking the element, the counter should be updated.
        assert_eq!(queue.get_count(), 1);

        // Perform the same check again and check that overall counter will become 2.
        queue.push_back("two".into());
        assert_eq!(queue.get_count(), 1);
        assert_eq!(queue.pop_front().unwrap(), "two");
        assert_eq!(queue.get_count(), 2);

        // Now attempt take no value, and check that counter is not increased.
        assert_eq!(queue.pop_front(), None);
        assert_eq!(queue.get_count(), 2);
    }
}
