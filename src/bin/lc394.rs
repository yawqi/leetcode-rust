use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut deq = s.chars().collect();
        Self::decode_helper(&mut deq)
    }

    fn decode_helper(deq: &mut VecDeque<char>) -> String {
        let mut s = String::new();
        let mut count = 0;
        while let Some(ch) = deq.pop_front() {
            match ch {
                ']' => break,
                '[' => {
                    let ret = Self::decode_helper(deq);
                    for _ in 0..count {
                        s.push_str(&ret);
                    }
                    count = 0;
                }
                '0'..='9' => {
                    count *= 10;
                    count += (ch as u32 - '0' as u32) as usize;
                }
                _ => s.push(ch),
            }
        }
        s
    }
}

fn main() {
    let res1 = Solution::decode_string("3[z]2[2[y]pq4[2[jk]e1[f]]]ef".to_owned());
    println!("{res1}");
}
