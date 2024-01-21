struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];

        let text1 = text1.chars().collect::<Vec<_>>();
        let text2 = text2.chars().collect::<Vec<_>>();
        for idx1 in 1..=text1.len() {
            for idx2 in 1..=text2.len() {
                if text1[idx1 - 1] == text2[idx2 - 1] {
                    dp[idx1][idx2] = std::cmp::max(
                        1 + dp[idx1 - 1][idx2 - 1],
                        std::cmp::max(dp[idx1][idx2 - 1], dp[idx1 - 1][idx2]),
                    );
                } else {
                    dp[idx1][idx2] = std::cmp::max(dp[idx1 - 1][idx2], dp[idx1][idx2 - 1]);
                }
            }
        }

        dp[text1.len()][text2.len()]
    }
}
