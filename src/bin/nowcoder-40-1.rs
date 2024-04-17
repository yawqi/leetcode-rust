fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut n = s.trim().parse::<usize>().unwrap();
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut n2 = s.trim().parse::<usize>().unwrap();
    if n == n2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
