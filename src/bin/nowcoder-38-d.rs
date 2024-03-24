fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = nums[0];
    let k = nums[1];

    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut idx = 0;
    let mut count = 0;

    let mut add_one = true;
    while idx < n - 1 {
        let v = (nums[idx] - nums[idx + 1]).abs();
        if v > k as i64 {
            count += (v - 1) / k as i64;
        } else if v == k as i64 {
            add_one = false;
        }
        idx += 1;
    }

    if add_one && count == 0 {
        count += 1;
    }
    println!("{}", count);
}
