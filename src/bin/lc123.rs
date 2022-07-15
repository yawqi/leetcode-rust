fn main() {
    Solution::max_profit(vec![1,2,4,2,5,7,2,4,9,0]);
}
struct Solution;

use std::collections::BinaryHeap;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut cur_min = prices[0];
        let mut cur_max = prices[0];
        let mut profits = BinaryHeap::with_capacity(2);
        let mut profit = 0;
        for price in prices {
            if price < cur_max {
                profits.push(cur_max - cur_min);
                cur_max = price;
                if price < cur_min {
                    cur_min = price;
                }
            }

            if price > cur_max {
                cur_max = price;
            }
        }
        profits.push(cur_max - cur_min);
        println!("{:?}", profits);
        for _ in 0..2 {
            let p = profits.pop().unwrap_or(0);
            println!("{:?}", p);
            profit += p;
        }
        profit
    }
}