use std::collections::HashMap;
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let mut map = HashMap::new();
    s.trim().chars().for_each(|c| {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
    });

    let xh = "xiaohong";
    for ch in xh.chars() {
        map.entry(ch).and_modify(|e| *e -= 1);
    }

    let mut ret = xh.to_owned();

    for (ch, count) in map {
        for _ in 0..count {
            ret.push(ch);
        }
    }
    println!("{}", ret);
}
