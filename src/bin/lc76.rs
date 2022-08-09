use std::collections::HashMap;
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut l = 0;
        let mut r = 0;
        let mut m1 = HashMap::new();
        t.chars()
            .for_each(|c| {
                let e = m1.entry(c).or_insert(0);
                *e += 1;
            });

        let mut m2 = HashMap::new();
        let mut s = s.chars().collect::<Vec<_>>();
        let mut ret = String::new();
        while r < s.len() {
            let e = m2.entry(&s[r]).or_insert(0);
            *e += 1;
            r += 1;
            while m1.iter().all(|(c, count)| {
                let e = m2.entry(c).or_insert(0);
                *e >= *count
            }) {
                if ret.len() == 0 || r - l < ret.len() {
                    ret = s[l..r].iter().collect();
                }
                let e = m2.entry(&s[l]).or_insert(0);
                *e -= 1;
                l += 1;
            }
        }
        ret
    }
}