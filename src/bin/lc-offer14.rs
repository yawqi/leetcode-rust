use std::collections::HashMap;
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut map = HashMap::new();
        s1.chars()
            .for_each(|c| {
                let e = map.entry(c).or_insert(0);
                *e += 1;
            });
        let s2 = s2.chars().collect::<Vec<_>>();

        let mut r = 0;
        let mut l = 0;
        let mut curr = HashMap::new();

        while r < s2.len() {
            let e = curr.entry(s2[r]).or_insert(0);
            *e += 1;
            r += 1;
            while r - l > s1.len() {
                let e = curr.entry(s2[l]).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    curr.remove(&s2[l]);
                }
                l += 1;
            }

            if curr == map {
                return true;
            }
        }
        false
    }
}