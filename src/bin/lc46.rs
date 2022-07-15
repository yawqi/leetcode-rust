use std::collections::HashSet;

struct Solution;
fn main() {}
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut paths = vec![];
        let mut path = vec![];
        let mut choosen = HashSet::new();
        Self::traverse(&mut path, &mut paths, &nums, &mut choosen);
        paths
    }

    fn traverse(path: &mut Vec<i32>, paths: &mut Vec<Vec<i32>>, nums: &Vec<i32>, choosen: &mut HashSet<i32>) {
        if nums.len() == choosen.len() {
            paths.push(path.clone());
            return;
        }

        for num in nums {
            if choosen.contains(num) {
                continue;
            }
            path.push(*num);
            choosen.insert(*num);
            Self::traverse(path, paths, nums, choosen);
            choosen.remove(num);
            path.pop();
        }
    }
}