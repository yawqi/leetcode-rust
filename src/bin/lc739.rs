struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut temp_stk = vec![];
        let mut days_stk = vec![0; temperatures.len()];
        for idx in 0..temperatures.len() {
            let temp = temperatures[idx];
            while let Some(prev_idx) = temp_stk.last() {
                if temperatures[*prev_idx] < temp {
                    days_stk[*prev_idx] = (idx - prev_idx) as i32;
                } else {
                    break;
                }
                temp_stk.pop();
            }
            temp_stk.push(idx);
        }
        days_stk
    }
}

fn main() {
    let temperatures_1 = vec![73, 74, 75, 71, 69, 72, 76, 73];
    let ret_1 = Solution::daily_temperatures(temperatures_1);
    println!("{:?}", ret_1);
}
