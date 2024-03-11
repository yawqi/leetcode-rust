#[derive(Clone)]
struct Fenwick {
    pub counts: Vec<i64>,
    len: usize,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self {
            counts: vec![0; n + 1],
            len: n,
        }
    }

    fn lowbit(idx: usize) -> usize {
        (idx & (!idx + 1))
    }

    pub fn modify(&mut self, mut idx: usize, value: i64) {
        while idx <= self.len {
            self.counts[idx] += value;
            idx += Self::lowbit(idx);
            // println!("{}", idx);
        }
    }

    fn get(&self, mut idx: usize) -> i64 {
        let mut sum = 0;
        while idx > 0 {
            sum += self.counts[idx];
            idx -= Self::lowbit(idx);
            // println!("{}", idx);
        }
        sum
    }

    pub fn range(&self, l_idx: usize, r_idx: usize) -> i64 {
        self.get(r_idx) - self.get(l_idx - 1)
    }
}

fn main() {
    let mut nq = String::new();
    let _ = std::io::stdin().read_line(&mut nq);
    let nq = nq
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = nq[0];
    let q = nq[1];

    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s);
    let mut chs = s.trim().chars().collect::<Vec<_>>();

    let combinations: Vec<Vec<_>> = vec![
        "red".chars().cycle().take(n).collect(),
        "rde".chars().cycle().take(n).collect(),
        "erd".chars().cycle().take(n).collect(),
        "edr".chars().cycle().take(n).collect(),
        "dre".chars().cycle().take(n).collect(),
        "der".chars().cycle().take(n).collect(),
    ];

    // println!("{} {} {:?}", n, q, combinations);
    let mut bits = vec![Fenwick::new(n); 6];
    for (i, comb) in combinations.iter().enumerate() {
        for (idx, (ch1, ch2)) in chs.iter().zip(comb.iter()).enumerate() {
            if *ch1 != *ch2 {
                bits[i].modify(idx + 1, 1);
            }
        }
    }

    let mut ans = vec![];
    for _ in 0..q {
        let mut op = String::new();
        let _ = std::io::stdin().read_line(&mut op);
        let op = op.trim().split(' ').collect::<Vec<_>>();
        let opcode = op[0].parse::<usize>().unwrap();
        let op1 = op[1].parse::<usize>().unwrap();
        match opcode {
            1 => {
                let op2 = op[2].chars().nth(0).unwrap();
                let ch = chs[op1 - 1];
                if ch == op2 {
                    continue;
                } else {
                    for (idx, bit) in bits.iter_mut().enumerate() {
                        if ch == combinations[idx][op1 - 1] {
                            bit.modify(op1, 1);
                        } else if op2 == combinations[idx][op1 - 1] {
                            bit.modify(op1, -1);
                        }
                    }
                    chs[op1 - 1] = op2;
                }
            }
            2 => {
                let op2 = op[2].parse::<usize>().unwrap();
                let mut least_ops = (op2 - op1 + 1) as i64;
                for bit in &bits {
                    least_ops = least_ops.min(bit.range(op1, op2));
                }
                ans.push(least_ops);
            }
            _ => {}
        }
    }

    for a in ans {
        println!("{}", a);
    }
}

// fn main() {
//     let mut nq = String::new();
//     let _ = std::io::stdin().read_line(&mut nq);
//     let nq = nq
//         .trim()
//         .split(' ')
//         .map(|n| n.parse::<usize>().unwrap())
//         .collect::<Vec<_>>();
//     let n = nq[0];
//     let q = nq[1];
//
//     let mut s = String::new();
//     let _ = std::io::stdin().read_line(&mut s);
//     let mut chs = s.trim().chars().collect::<Vec<_>>();
//     let combinations: Vec<Vec<_>> = vec![
//         "red".chars().collect(),
//         "rde".chars().collect(),
//         "erd".chars().collect(),
//         "edr".chars().collect(),
//         "dre".chars().collect(),
//         "der".chars().collect(),
//     ];
//     let mut ans = vec![];
//     for _ in 0..q {
//         let mut op = String::new();
//         let _ = std::io::stdin().read_line(&mut op);
//         let op = op.trim().split(' ').collect::<Vec<_>>();
//         let opcode = op[0].parse::<usize>().unwrap();
//         let op1 = op[1].parse::<usize>().unwrap() - 1;
//         match opcode {
//             1 => {
//                 let op2 = op[2].chars().nth(0).unwrap();
//                 chs[op1] = op2;
//             }
//             2 => {
//                 let op2 = op[2].parse::<usize>().unwrap() - 1;
//                 let selected = &chs[op1..=op2];
//                 let mut least_ops = op2 - op1 + 1;
//                 for comb in &combinations {
//                     let ret = comb
//                         .iter()
//                         .cycle()
//                         .take(op2 - op1 + 1)
//                         .zip(selected.iter())
//                         .fold(0, |a, (v1, v2)| if *v1 != *v2 { a + 1 } else { a });
//                     least_ops = least_ops.min(ret);
//                 }
//                 ans.push(least_ops);
//             }
//             _ => {}
//         }
//     }
//     for a in ans {
//         println!("{}", a);
//     }
// }
