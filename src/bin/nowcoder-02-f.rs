fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = nums[0];
    let m = nums[1];

    let mut ns = vec![];
    let mut ms = vec![];

    let mut n_ones = 0;
    let mut n_twos = 0;
    let mut m_ones = 0;
    let mut m_twos = 0;
    for _ in 0..n {
        s.clear();
        std::io::stdin().read_line(&mut s);
        let val = s.trim().parse::<usize>().unwrap();
        ns.push(val);
        if val == 1 {
            n_ones += 1;
        } else {
            n_twos += 1;
        }
    }

    for _ in 0..m {
        s.clear();
        std::io::stdin().read_line(&mut s);
        let val = s.trim().parse::<usize>().unwrap();
        ms.push(val);
        if val == 1 {
            m_ones += 1;
        } else {
            m_twos += 1;
        }
    }
}
