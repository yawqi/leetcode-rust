struct Solution;

impl Solution {
    pub fn count_key_changes(s: String) -> i32 {
        let s = s.to_lowercase().chars().collect::<Vec<_>>();
        let mut prev = s[0];
        let mut count = 0;
        for idx in 1..s.len() {
            if prev != s[idx] {
                prev = s[idx];
                count += 1;
            }
        }
        count
    }
}
