struct Solution;

use std::collections::BTreeMap;
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counts = BTreeMap::new();
        word.chars().for_each(|ch| {
            counts.entry(ch).and_modify(|v| *v += 1).or_insert(1);
        });

        let mut presses = 0;
        let mut kinds = 0;
        let mut counts = counts.values().collect::<Vec<_>>();
        counts.sort();
        for v in counts.into_iter().rev() {
            presses += (1 + kinds / 8) * *v;
            kinds += 1;
        }

        presses
    }
}

fn main() {
    let s = "abzaqsqcyrbzsrvamylmyxdjl".to_owned();
    Solution::minimum_pushes(s);
}
