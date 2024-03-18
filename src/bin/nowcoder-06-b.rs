fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = nums[0];
    let mut x = nums[1];
    s.clear();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();

    let mut idx = match nums.binary_search(&x) {
        Ok(v) => v,
        Err(v) => v,
    };
    while idx < nums.len() && nums[idx] <= x {
        idx += 1;
    }

    if idx > 0 {
        x -= nums[idx - 1];
    }

    println!("{} {}", idx, x);
}
