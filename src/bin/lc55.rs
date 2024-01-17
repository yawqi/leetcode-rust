struct Solution;

impl Solution {
    // pub fn can_jump(nums: Vec<i32>) -> bool {
    //     let mut jumapble = vec![false; nums.len()];
    //     jumapble[0] = true;
    //     for i in 0..jumapble.len() {
    //         if !jumapble[i] {
    //             continue;
    //         }
    //         for nxt_jmp in 1..=nums[i] {
    //             if nxt_jmp + i >= jumapble.len() {
    //                 break;
    //             }
    //             jumapble[i + nxt_jmp] = true;
    //         }
    //     }

    //     *jumapble.last().unwrap()
    // }

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut furthest = 0;
        let mut curr = 0;
        let len = nums.len();
        while curr <= furthest && curr < len {
            furthest = furthest.max(curr + nums[curr] as usize);
            curr += 1;
        }

        furthest >= len - 1
    }
}
