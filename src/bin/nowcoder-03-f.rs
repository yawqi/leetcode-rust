const POW: usize = 1000000007;
fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s1);
    std::io::stdin().read_line(&mut s2);

    let mut nums = s2
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut ones = 0;
    let mut twos = 0;
    let mut threes = 0;
    for num in nums {
        match num {
            1 => ones += 1,
            2 => twos += 1,
            3 => threes += 1,
            _ => {}
        };
    }

    let mut counts_3 = 1;
    let mut comb = 1;
    for i in 1..=threes {
        comb *= (threes - i + 1);
        comb /= i;
        comb %= POW;
        counts_3 += (i + 1) * comb;
        counts_3 %= POW;
    }

    let mut comb = 1;
    let mut ans = counts_3;
    for i in 1..=twos {
        comb *= (twos - i + 1);
        comb /= i;
        comb %= POW;
        ans += (i + 1) * comb * counts_3;
        ans %= POW;
    }

    ans = (2_usize.pow(ones) * ans) % POW;
    ans = ((ans - 1) + POW) % POW;
    println!("{}", ans);
}
