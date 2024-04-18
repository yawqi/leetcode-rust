struct CirculalQueue {
    queue: Vec<i64>,
    size: usize,
    head: usize,
    tail: usize,
}

impl CirculalQueue {
    pub fn new(sz: usize) -> Self {
        Self {
            size: sz + 1,
            queue: vec![0; sz + 1],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, x: i64) {
        if self.is_full() {
            return;
        }
        self.queue[self.tail % self.size] = x;
        self.tail = (self.tail + 1) % self.size;
    }

    pub fn pop(&mut self) -> Option<i64> {
        if self.is_empty() {
            return None;
        }
        let v = self.queue[self.head % self.size];
        self.head = (self.head + 1) % self.size;
        Some(v)
    }

    pub fn front(&self) -> Option<i64> {
        if self.is_empty() {
            return None;
        }

        Some(self.queue[self.head % self.size])
    }

    fn is_full(&self) -> bool {
        self.head % self.size == (self.tail + 1) % self.size
    }

    fn is_empty(&self) -> bool {
        self.head % self.size == self.tail % self.size
    }
}
fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let nums = n
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut q = CirculalQueue::new(nums[0]);
    for _ in 0..nums[1] {
        let mut op = String::new();
        std::io::stdin().read_line(&mut op);
        let op = op.trim().split(' ').collect::<Vec<_>>();

        match op[0] {
            "front" => {
                if let Some(v) = q.front() {
                    println!("{}", v);
                } else {
                    println!("empty");
                }
            }
            "pop" => {
                if let Some(v) = q.pop() {
                    println!("{}", v);
                } else {
                    println!("empty");
                }
            }
            "push" => {
                q.push(op[1].parse::<i64>().unwrap());
            }
            _ => {}
        }
    }
}
