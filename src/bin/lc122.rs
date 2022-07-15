fn main() {}
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cur_min = prices[0];
        let mut cur_max = prices[0];
        let mut profit = 0;
        for price in prices {
            if price < cur_max {
                profit += cur_max - cur_min;
                cur_max = price;
                cur_min = price;
            }

            if price > cur_max {
                cur_max = price;
            }
        }
        profit += cur_max - cur_min;
        profit
    }
}