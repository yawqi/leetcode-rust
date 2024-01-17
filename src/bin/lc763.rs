use std::{collections::HashMap, ops::Index};
struct Solution;
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurrences = HashMap::new();
        let mut ret = vec![];
        s.chars().enumerate().for_each(|(idx, ch)| {
            last_occurrences
                .entry(ch)
                .and_modify(|e| *e = idx)
                .or_insert(idx);
        });

        let mut curr_boundary = 0;
        let mut prev_boundary = 0;
        let s = s.chars().collect::<Vec<_>>();
        for i in 0..s.len() {
            let last_occur = *last_occurrences.get(&s[i]).unwrap();
            if last_occur >= curr_boundary {
                curr_boundary = last_occur;
            }

            if i == curr_boundary {
                ret.push((curr_boundary - prev_boundary) as i32);
                prev_boundary = curr_boundary;
            }
        }
        ret[0] += 1;
        ret
    }
}
