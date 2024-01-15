use std::os::unix::process::parent_id;

struct Solution;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = Heap::new(nums);
        heap.heap_sort(k as usize);
        heap.heap[heap.heap.len() - k as usize]
    }
}

struct Heap {
    heap: Vec<i32>,
}

impl Heap {
    fn new(nums: Vec<i32>) -> Self {
        Self { heap: nums }
    }

    fn max_heapify(&mut self, mut parent: usize, size: usize) {
        let mut son = 2 * parent + 1;
        while son < size {
            if son + 1 < size && self.heap[son + 1] > self.heap[son] {
                son += 1;
            }

            if self.heap[son] >= self.heap[parent] {
                self.heap.swap(parent, son);
                parent = son;
                son = 2 * parent + 1;
            } else {
                break;
            }
        }
    }

    fn heap_sort(&mut self, k: usize) {
        for i in (0..=(self.heap.len() / 2 - 1)).rev() {
            self.max_heapify(i, self.heap.len());
        }

        for kk in 1..=k {
            self.heap.swap(0, self.heap.len() - kk);
            self.max_heapify(0, self.heap.len() - kk);
        }
    }
}
