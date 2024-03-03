fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let n = n.trim().parse::<usize>().unwrap();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums);
    let mut nums = nums
        .trim()
        .split(' ')
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    nums.sort();
    let mut v1 = vec![];
    let mut v2 = vec![];
    for i in 0..n {
        let idx1 = 2 * i;
        let idx2 = idx1 + 1;
        if nums[idx1] != nums[idx2] {
            println!("-1");
            return;
        }
        v1.push(nums[idx1]);
        v2.push(nums[idx2]);
    }

    for v in v1 {
        print!("{} ", v);
    }
    println!();
    for v in v2 {
        print!("{} ", v);
    }
    println!();
}
