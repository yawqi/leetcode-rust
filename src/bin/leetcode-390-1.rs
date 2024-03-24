struct Solution;
impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let mut l = 0;
        let mut r = 0;
        let mut counts = vec![0; 26];
        let mut max_len = 0;
        while r < s.len() {
            let mut idx = (s[r] as u8 - 'a' as u8) as usize; 
            counts[idx] += 1;
            while l < r && counts[idx] > 2 {
            let nxt_idx = (s[l] as u8 - 'a' as u8) as usize; 
            l += 1;
            counts[nxt_idx] -= 1;
            }
            r += 1;
            max_len = max_len.max(r - l);
        }
        max_len as i32
    }
}
