use std::cmp::{max, min};
struct Solution;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut left = vec![0; heights.len()];
        let mut right = vec![0; heights.len()];
        let mut max_height = 0;
        let mut min_stack = vec![];
        for idx in 0..heights.len() {
            let height = heights[idx];
            while let Some(&prev_idx) = min_stack.last() {
                if heights[prev_idx] >= height {
                    min_stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&prev_idx) = min_stack.last() {
                left[idx] = prev_idx as i32;
            } else {
                left[idx] = -1;
            }
            min_stack.push(idx);
        }

        min_stack = vec![];
        for idx in (0..heights.len()).rev() {
            let height = heights[idx];
            while let Some(&prev_idx) = min_stack.last() {
                if heights[prev_idx] >= height {
                    min_stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&prev_idx) = min_stack.last() {
                right[idx] = prev_idx as i32;
            } else {
                right[idx] = heights.len() as i32;
            }
            min_stack.push(idx);
        }

        for i in 0..heights.len() {
            max_height = max(max_height, heights[i] * (right[i] - left[i] - 1));
        }
        max_height
    }
}
