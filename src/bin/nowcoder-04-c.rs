fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut v = s.trim().chars().collect::<Vec<_>>();
    let mut word_beg = true;
    let mut count = 0;
    for ch in v {
        if word_beg {
            word_beg = false;
            continue;
        }
        if ch.is_uppercase() {
            count += 1;
            word_beg = true;
        }
    }
    println!("{}", count);
}
