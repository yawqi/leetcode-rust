fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let v = s
        .trim()
        .chars()
        .map(|c| (c as u8 - '0' as u8) as u64)
        .collect::<Vec<_>>();
    let mut acc = 0;
    let mut count = 0;
    for n in v {
        acc += n;
        if acc != 0 && acc % 9 == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
