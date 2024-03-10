fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let n = n.trim().parse::<u32>().unwrap();
    println!("{}", n / 1000);
}
