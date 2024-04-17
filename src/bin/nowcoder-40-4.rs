fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = nums[0];
    let coins = nums[1];
    let mut weapons = vec![];
    let mut res = 0;

    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let mut weapon = s
            .trim()
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if weapon[1] > coins {
            continue;
        }
        weapons.push(weapon);
    }

    let mut dp = vec![vec![0; coins + 1]; 2];
    let mut res = 0;
    for w in weapons {
        for curr_coin in 1..=coins {
            for level in 0..=w[4] {
                let cost = w[1] + w[2] * level;
                if cost > curr_coin {
                    break;
                }
                let att = w[0] + w[3] * level;
                dp[1][curr_coin] = std::cmp::max(dp[0][curr_coin], dp[0][curr_coin - cost] + att);
                res = std::cmp::max(res, dp[1][curr_coin]);
            }
            dp[0] = dp[1].clone();
        }
    }
    println!("{}", res);
}
