use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        if let Some(med) = self.max_heap.peek() {
            if num > *med {
                self.min_heap.push(Reverse(num));
                if self.min_heap.len() > self.max_heap.len() {
                    let nxt_med = self.min_heap.pop().unwrap();
                    self.max_heap.push(nxt_med.0);
                }
            } else {
                self.max_heap.push(num);
                if self.max_heap.len() > self.min_heap.len() + 1 {
                    let nxt = self.max_heap.pop().unwrap();
                    self.min_heap.push(Reverse(nxt));
                }
            }
        } else {
            self.max_heap.push(num);
        }
    }

    fn find_median(&self) -> f64 {
        if self.min_heap.len() == self.max_heap.len() {
            let med1 = *self.max_heap.peek().unwrap();
            let med2 = self.min_heap.peek().unwrap().0;
            (med1 as f64 + med2 as f64) / 2.0
        } else {
            *self.max_heap.peek().unwrap() as f64
        }
    }
}
