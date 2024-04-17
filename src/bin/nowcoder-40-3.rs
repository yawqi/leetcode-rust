fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut n = s.trim().parse::<usize>().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut a = vec![0; n];
    let mut b = vec![0; n];
    let mut a_set = vec![false; n + 1];
    let mut b_set = vec![false; n + 1];
    for (idx, v) in nums.into_iter().enumerate() {
        if !a_set[v] {
            a[idx] = v;
            a_set[v] = true;
        } else if !b_set[v] {
            b[idx] = v;
            b_set[v] = true;
        } else {
            println!("-1");
            return;
        }
    }

    let mut a_beg = 0;
    let mut a_v = 1;
    while a_beg < n {
        if a[a_beg] == 0 {
            while a_v <= n && a_set[a_v] {
                a_v += 1;
            }
            a_set[a_v] = true;
            a[a_beg] = a_v;
        }
        a_beg += 1;
    }

    let mut b_beg = 0;
    let mut b_v = 1;
    while b_beg < n {
        if b[b_beg] == 0 {
            while b_v <= n && b_set[b_v] {
                b_v += 1;
            }
            b_set[b_v] = true;
            b[b_beg] = b_v;
        }
        b_beg += 1;
    }

    for v in a {
        print!("{} ", v);
    }
    println!("");
    for v in b {
        print!("{} ", v);
    }
    println!("");
}
