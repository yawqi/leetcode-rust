struct Solution;

impl Solution {
    pub fn longest_valid_parentheses_1(s: String) -> i32 {
        let mut longest = 0;
        let mut dp = vec![0; s.len() + 1];
        let s = s.chars().collect::<Vec<_>>();

        for idx in 2..=s.len() {
            if s[idx - 1] == ')' {
                if s[idx - 2] == '(' {
                    dp[idx] = dp[idx - 2] + 2;
                } else if idx >= dp[idx - 1] {
                    let prev = idx - 1 - dp[idx - 1];
                    if prev >= 1 && s[prev - 1] == '(' {
                        dp[idx] = dp[idx - 1] + 2 + dp[prev - 1];
                    }
                }
                longest = longest.max(dp[idx]);
            }
        }
        longest as i32
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stk = vec![-1];
        let mut ret = 0;
        s.chars().enumerate().for_each(|(idx, ch)| {
            if ch == '(' {
                stk.push(idx as i32);
            } else {
                let prev = stk.pop().unwrap();
                if stk.is_empty() {
                    ret = ret.max(idx as i32 - prev - 1);
                    stk.push(idx as i32);
                } else {
                    ret = ret.max(idx as i32 - prev + 1);
                }
            }
        });
        ret
    }
}
