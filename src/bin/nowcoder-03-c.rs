fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let n = nums[0] as usize;
    let k = nums[1];

    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    nums.sort();

    let mut l = 0;
    let mut r = 0;
    let mut max_count = 0;
    while r < n {
        while l <= r && nums[l] + k < nums[r] {
            l += 1;
        }
        max_count = max_count.max(r - l + 1);
        r += 1;
    }

    let res = (max_count as f32 / n as f32);
    println!("{}", res);
}
