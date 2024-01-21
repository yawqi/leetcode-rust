impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut dps = vec![vec![false; s.len()]; s.len() + 1];
        let s = s.chars().collect::<Vec<_>>();
        let mut ret = s[0].to_string();
        let slen = s.len();
        for idx in 0..s.len() {
            dps[0][idx] = true;
            dps[1][idx] = true;
        }

        for len in 2..=s.len() {
            for idx in 0..=slen - len {
                if dps[len - 2][idx + 1] && s[idx] == s[idx + len - 1] {
                    dps[len][idx] = true;
                    ret = s[idx..idx + len].iter().collect();
                }
            }
        }
        ret
    }
}
