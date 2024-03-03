fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let s = s.trim().clone();
    let (s1, s2) = s.split_at(s.len() / 2);
    println!("{s1}\n{s2}");
}
