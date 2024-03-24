fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut n = s.trim().parse::<usize>().unwrap();
    let mut count = 0;
    while n % 10 != 0 {
        n += 1;
        count += 1;
    }
    println!("{}", count);
}
