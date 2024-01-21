fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let v = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    if v[0] < v[1] {
        println!("kou")
    } else if v[0] > v[1] {
        println!("yukari");
    } else {
        println!("draw");
    }
}
