fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let t = nums[0];
    let mut ans = vec![];

    for _ in 0..t {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let n = s.trim().parse::<usize>().unwrap();
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let nums = s
            .trim()
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
    }

    for a in ans {
        println!("{}", ans);
    }
}
