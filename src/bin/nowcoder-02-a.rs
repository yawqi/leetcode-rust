fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let s = s
        .trim()
        .chars()
        .enumerate()
        .filter(|(idx, c)| *idx == 1)
        .map(|(_, c)| c)
        .collect();
    println!("{}", s);
}
