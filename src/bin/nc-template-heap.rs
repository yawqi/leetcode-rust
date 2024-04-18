struct MaxHeap {
    v: Vec<i64>,
    size: usize,
}

impl MaxHeap {
    pub fn new() -> Self {
        Self {
            v: vec![0; 100001],
            size: 0,
        }
    }

    pub fn push(&mut self, n: i64) {
        self.v[self.size] = n;
        let mut index = self.size;
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.v[parent] < n {
                self.v.swap(parent, index);
                index = parent;
            } else {
                break;
            }
        }
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<i64> {
        if self.size == 0 {
            return None;
        }
        let root_value = self.v[0];
        self.v.swap(0, self.size - 1);
        self.size -= 1;
        self.heapify(0);
        Some(root_value)
    }

    pub fn top(&self) -> Option<i64> {
        match self.size {
            0 => None,
            _ => Some(self.v[0]),
        }
    }

    fn heapify(&mut self, mut index: usize) {
        let lchild = 2 * index + 1;
        let rchild = 2 * index + 2;
        let mut curr = self.v[index];
        let mut curr_index = index;
        if lchild < self.size {
            if curr < self.v[lchild] {
                curr = self.v[lchild];
                curr_index = lchild;
            }
        }
        if rchild < self.size {
            if curr < self.v[rchild] {
                curr = self.v[rchild];
                curr_index = rchild;
            }
        }

        if index != curr_index {
            self.v.swap(index, curr_index);
            self.heapify(curr_index);
        }
    }
}

fn main() {
    let mut nop = String::new();
    std::io::stdin().read_line(&mut nop);
    let nop = nop.trim().parse::<usize>().unwrap();
    let mut max_heap = MaxHeap::new();
    for _ in 0..nop {
        let mut op = String::new();
        std::io::stdin().read_line(&mut op);
        let op = op.trim().split(' ').collect::<Vec<_>>();

        match op[0] {
            "push" => {
                let n = op[1].parse::<i64>().unwrap();
                max_heap.push(n);
            }
            "top" => match max_heap.top() {
                None => {
                    println!("empty");
                }
                Some(v) => {
                    println!("{}", v);
                }
            },
            "pop" => match max_heap.pop() {
                None => {
                    println!("empty");
                }
                Some(v) => {
                    println!("{}", v);
                }
            },
            _ => {}
        }
    }
}
