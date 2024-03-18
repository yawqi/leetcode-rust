fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (n1, n2) = (nums[0], nums[1]);
    if n1 + n2 >= 10 {
        println!("No");
    } else {
        println!("Yes");
    }
}
