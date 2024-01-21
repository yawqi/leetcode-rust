fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut days = String::new();
    let mut nights = String::new();

    std::io::stdin().read_line(&mut days).unwrap();
    std::io::stdin().read_line(&mut nights).unwrap();

    let mut days = days.trim().chars().map(|s| s == 'Y').collect::<Vec<_>>();
    let mut nights = nights.trim().chars().map(|s| s == 'Y').collect::<Vec<_>>();

    let mut counts = 0;
    for i in 0..n {
        if days[i] || nights[i] {
            counts += 2;
        }

        if days[i] && nights[i] {
            counts += 1;
        }
    }

    println!("{counts}");
}
