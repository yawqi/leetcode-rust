fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let mut n = n.trim().parse::<usize>().unwrap();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums).unwrap();
    let mut nums = nums
        .trim()
        .split(' ')
        .enumerate()
        .map(|(idx, v)| (v.parse::<u64>().unwrap(), idx))
        .collect::<Vec<_>>();

    nums.sort();
    let mut ret = vec![0.0; n];
    for idx in 0..n {
        let pos = nums[idx].1;
        ret[pos] = get_median(&nums, idx);
    }
    ret.into_iter().for_each(|v| println!("{}", v));
}

fn get_median(nums: &Vec<(u64, usize)>, idx: usize) -> f64 {
    let origin_median_1 = (nums.len() - 1) / 2;
    let origin_median_2 = nums.len() / 2;
    let mut res = 0.0;
    if nums.len() % 2 == 0 {
        if idx <= origin_median_1 {
            res = nums[origin_median_2].0 as f64;
        } else {
            res = nums[origin_median_1].0 as f64;
        }
    } else {
        if idx < origin_median_1 {
            res = (nums[origin_median_1].0 + nums[origin_median_1 + 1].0) as f64 / 2.0;
        } else if idx > origin_median_1 {
            res = (nums[origin_median_1 - 1].0 + nums[origin_median_1].0) as f64 / 2.0;
        } else {
            res = (nums[origin_median_1 - 1].0 + nums[origin_median_1 + 1].0) as f64 / 2.0;
        }
    }

    res
}
