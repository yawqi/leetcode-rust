fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = nums[0];
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let s = s.trim().chars().collect::<Vec<_>>();
    let first = s.first().unwrap();
    let last = s.last().unwrap();
    if first > last {
        println!("{}", first);
    } else {
        println!("{}", last);
    }
}
