impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = vec![-1, -1];
        if nums.len() == 0 {
            return ret;
        }
        ret[0] = left_bound(&nums, target);
        ret[1] = right_bound(&nums, target);
        ret
    }
}

pub fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if left < nums.len() && nums[left] == target {
        left as i32
    } else {
        -1
    }
}

pub fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut left = 0;
    let mut right = nums.len();
    while left < right {
        let mid = left + (right - left) / 2;
        if nums[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;;
        }
    }

    if left >= 1 && nums[left - 1] == target {
        left as i32 - 1
    } else {
        -1
    }
}