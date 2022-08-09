use std::collections::HashMap;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut ret = 0;
        let mut map = HashMap::new();
        let s = s.chars().collect::<Vec<_>>();
        while r < s.len() {
            let c = s[r];
            let e = map.entry(c).or_insert(0);
            *e += 1;
            r += 1;
            while *map.get(&c).unwrap() > 1 {
                let c2 = s[l];
                let count = map.get(&c2).unwrap();
                map.insert(c2, count-1);
                l += 1;
            }
            ret = std::cmp::max(ret, r-l);
        }
        ret as i32
    }
}