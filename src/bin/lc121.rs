fn main() {

}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut cur_max = prices[0];
        // let mut cur_min = prices[0];
        // let mut profit = 0;
        // for price in prices {
        //     if price < cur_min {
        //         cur_min = price;
        //         cur_max = price;
        //     }

        //     if price > cur_max {
        //         cur_max = price;
        //         profit = std::cmp::max(cur_max - cur_min, profit);
        //     }
        // }
        // profit
        let mut profit = 0;
        let mut dp = vec![vec![0; 2]; prices.len() + 1];
        dp[0][1] = i32::MIN;
        for i in 1..=prices.len() {
            dp[0][1] = 
        }
        dp[prices.len()][0]
    }
}
