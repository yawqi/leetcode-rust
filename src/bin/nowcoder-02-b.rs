fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut s = s.trim().chars().collect::<Vec<_>>();
    s.sort();
    let mut target = 0;
    for i in 0..s.len() {
        if s[i] != '0' {
            target = i;
            break;
        }
    }
    s.swap(0, target);
    let num = s.into_iter().collect::<String>().parse::<u32>().unwrap();
    println!("{num}");
}
