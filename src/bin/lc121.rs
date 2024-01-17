fn main() {}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut min_price = prices[0];
        for i in 0..prices.len() {
            min_price = min_price.min(prices[i]);
            profit = profit.max(prices[i] - min_price);
        }
        profit
    }
}
