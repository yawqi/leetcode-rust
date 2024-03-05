const MOD: u64 = 1000000007;
fn qpow(mut base: u64, mut pow: u64) -> u64 {
    let mut res = 1;
    while pow != 0 {
        if pow & 1 != 0 {
            res = res * base % MOD;
        }
        base = base * base % MOD;
        pow >>= 1;
    }
    res
}

fn inv(num: u64) -> u64 {
    qpow(num, MOD - 2)
}

fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s1);
    std::io::stdin().read_line(&mut s2);

    let mut nums = s2
        .trim()
        .split(' ')
        .map(|v| v.parse::<u64>().unwrap())
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
        comb = ((threes - i + 1) * comb) % MOD;
        comb = (comb * inv(i)) % MOD;
        counts_3 = (counts_3 + (i + 1) * comb) % MOD;
    }

    comb = 1;
    let mut ans = counts_3;
    for i in 1..=twos {
        comb = (comb * (twos - i + 1)) % MOD;
        comb = (comb * inv(i)) % MOD;
        ans = (ans + (i + 1) * comb % MOD * counts_3 % MOD) % MOD;
    }

    ans = (qpow(2, ones) % MOD) * ans % MOD;
    ans = ((ans - 1) + MOD) % MOD;
    println!("{}", ans);
}
