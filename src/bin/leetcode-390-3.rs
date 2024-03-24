struct Solution;

use std::collections::{BTreeMap, HashMap, HashSet};
impl Solution {
    pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![0i64; nums.len()];
        let mut counts = vec![0i64; 100001];
        let mut map = vec![0i64; 100001];
        let mut counts = HashMap::new();
        counts.insert(0, 100001);
        let mut curr_ans = 0;
        for i in 0..nums.len() {
            let id = nums[i] as usize;
            let f = freq[i];
            let prev_count = map[id] as usize;
            map[id] += f as i64;
            let nxt_count = map[id] as usize;
            let nxt_e = counts.entry(nxt_count).or_default();
            *nxt_e += 1;
            let prev_e = counts.entry(prev_count).or_default();
            *prev_e -= 1;
            let curr_count = *prev_e;
            if curr_count == 0 {
                counts.remove(&prev_count);
            }
            if prev_count <= nxt_count {
                curr_ans = curr_ans.max(nxt_count);
            } else if prev_count == curr_ans && curr_count == 0 {
                curr_ans = 0;
                for (k, v) in &counts {
                    if *v != 0 {
                        curr_ans = curr_ans.max(*k);
                    }
                }
            }
            ans[i] = curr_ans as i64;
        }

        ans
    }

    pub fn most_frequent_i_ds_new(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
        let mut ans = vec![];
        let mut id_counts = vec![0i64; 100001];
        let mut count_to_ids = BTreeMap::new();

        count_to_ids.insert(0, (1..100001).collect());
        id_counts[0] = 100001;
        for idx in 0..nums.len() {
            let id = nums[idx] as usize;
            let fq = freq[idx] as usize;

            let prev_count = id_counts[id];
            id_counts[id] += fq as i64;
            let curr_count = id_counts[id];
            let prev_e = count_to_ids.entry(prev_count).or_insert(HashSet::new());
            prev_e.remove(&id);
            if prev_e.len() == 0 {
                count_to_ids.remove(&prev_count);
            }
            let curr_e = count_to_ids.entry(curr_count).or_insert(HashSet::new());
            curr_e.insert(id);

            ans.push(*count_to_ids.last_key_value().unwrap().0);
        }
        ans
    }
}

fn main() {
    unimplemented!();
}
