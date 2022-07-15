use std::collections::{HashSet, VecDeque};

struct Solution;
fn main() {
    Solution::open_lock(vec!["8888".to_string()],"0009".to_string());
}

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let target = target.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();
        let deadends: HashSet<_> = deadends
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as i32)
                    .collect::<Vec<i32>>()
            })
            .collect();
        if deadends.contains(&vec![0i32; 4]) {
            return -1;
        }
        let mut steps = 0;
        let mut deque = VecDeque::new();
        let mut choosen = HashSet::new();
        choosen.insert(vec![0i32; 4]);
        deque.push_back(vec![0i32; 4]);
        while !deque.is_empty() {
            let len = deque.len();
            for _ in 0..len {
                let curr = deque.pop_front().unwrap();
                if curr == target {
                    return steps;
                }
                for i in 0..4 {
                    let mut plus_one = curr.clone();
                    let mut minus_one = curr.clone();
                    plus_one[i] = (curr[i] + 1) % 10;
                    minus_one[i] = ((curr[i] - 1) + 10) % 10;
                    println!("{:?}", minus_one);
                    if !deadends.contains(&plus_one) && !choosen.contains(&plus_one) {
                        choosen.insert(plus_one.clone());
                        deque.push_back(plus_one);
                    }

                    if !deadends.contains(&minus_one) && !choosen.contains(&minus_one) {
                        choosen.insert(minus_one.clone());
                        deque.push_back(minus_one);
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}