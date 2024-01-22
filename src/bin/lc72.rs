struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1 = word1.len();
        let len2 = word2.len();
        let word1 = word1.chars().collect::<Vec<_>>();
        let word2 = word2.chars().collect::<Vec<_>>();

        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            dp[i][0] = i;
        }

        for j in 0..=len2 {
            dp[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = std::cmp::min(
                        dp[i - 1][j - 1],
                        std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                    );
                } else {
                    dp[i][j] = 1 + std::cmp::min(
                        dp[i - 1][j - 1],
                        std::cmp::min(dp[i - 1][j], dp[i][j - 1]),
                    );
                }
            }
        }

        dp[len1][len2] as i32
    }
}

fn main() {
    Solution::min_distance("horse".to_owned(), "ros".to_owned());
}
