fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);

    let mut svec = s.chars().collect::<Vec<_>>();
    let len = svec.len();
    if len <= 3 {
        println!("-1");
        return;
    }

    let mut c1 = svec[0];
    let mut target = 0;
    let mut c2 = svec[0];
    for idx in 1..=len / 2 {
        if c1 != svec[idx] {
            target = idx;
            break;
        }
    }
    if target == 0 {
        println!("-1");
    } else {
        svec.swap(0, target);
        svec.swap(len - 1, len - target - 1);
        println!("{}", svec.into_iter().collect::<String>());
    }
}
