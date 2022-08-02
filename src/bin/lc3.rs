use std::collections::HashSet;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut set = HashSet::new();
        let mut l = 0;
        let mut r = 0;
        let mut max = 0;
        let mut s = s.chars().collect::<Vec<_>>();
        while r < s.len() {
            if set.contains(&s[r]) {
                max = std::cmp::max(max, set.len());
                while s[l] != s[r] {
                    set.remove(&s[l]);
                    l += 1;
                }
                l += 1;
            } else {
                set.insert(s[r]);
            }
            r += 1;
        }
        max = std::cmp::max(max, set.len());
        max as i32
    }
}