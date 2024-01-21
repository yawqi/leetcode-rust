fn main() {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).unwrap();
    let x = x.trim().parse::<u64>().unwrap();

    let primes = find_primes(x);
    let len = primes.len();
    let mut ans = vec![];
    for idx in 0..len {
        if x % primes[idx] != 0 {
            continue;
        }

        ans.push(primes[idx]);
        if dfs(x / primes[idx], &primes, idx, &mut ans) {
            break;
        }
        ans.pop();
    }

    if ans.is_empty() {
        println!("-1");
    } else {
        println!("{}", ans.len());
        for num in ans {
            print!("{} ", num);
        }
    }
}

fn find_primes(n: u64) -> Vec<u64> {
    let mut primes = vec![];
    if n < 2 {
        return primes;
    }
    primes.push(2);

    let mut value = 2;
    'outer: loop {
        value += 1;
        if value > n {
            break;
        }
        for v in &primes {
            if value % v == 0 {
                continue 'outer;
            }
        }
        primes.push(value);
    }

    primes
}

fn dfs(target: u64, primes: &Vec<u64>, curr: usize, ans: &mut Vec<u64>) -> bool {
    if target == 1 {
        return true;
    }

    let len = primes.len();
    for idx in 0..len {
        if curr == idx || target % primes[idx] != 0 {
            continue;
        }

        ans.push(primes[idx]);
        if dfs(target / primes[idx], primes, idx, ans) {
            return true;
        }
        ans.pop();
    }
    false
}
