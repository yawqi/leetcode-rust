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
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut dp = vec![200001; 128];
        dp[127] = 0;

        for v in nums {
            let mut ndp = dp.clone();
            for pv in 1..=127 {
                ndp[(v & pv) as usize] = std::cmp::min(dp[pv as usize] + 1, ndp[(v & pv) as usize]);
            }
            dp = ndp;
        }
        if dp[0] >= 200001 {
            dp[0] = -1;
        } else {
            dp[0] = n as i32 - dp[0];
        }
        ans.push(dp[0]);
    }

    for a in ans {
        println!("{}", a);
    }
}
