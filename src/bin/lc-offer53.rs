impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let r = right_bound(&nums, target);
        let l = left_bound(&nums, target);
        if l == -1 {
            0
        } else {
            r - l + 1
        }
    }
}

pub fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    if r == 0 { return -1; }
    while l < r {
        let mid = l + (r - l) / 2;
        if nums[mid] >= target {
            r = mid;
        } else {
            l = mid + 1;
        }
    }

    if l < nums.len() && nums[l] == target {
        l as i32
    } else {
        -1
    }
}

pub fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len();
    if r == 0 { return -1; }
    while l < r {
        let mid = l + (r - l) / 2;
        if nums[mid] <= target {
            l = mid + 1;
        } else {
            r = mid;
        }
    }

    if l >= 1 && nums[l - 1] == target {
        l as i32 - 1
    } else {
        -1
    }
}