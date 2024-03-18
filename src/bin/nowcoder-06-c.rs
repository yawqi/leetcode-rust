fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut n = nums[0];
    if n % 495 == 0 {
        println!("-1");
        return;
    }

    let mut append_count = 1;
    let mut upper_helper = 10;
    loop {
        n *= 10;
        let lower_bound = n;
        let higher_bound = lower_bound + upper_helper - 1;
        if lower_bound % 495 == 0 || higher_bound % 495 == 0 {
            let v = vec!['0'; append_count];
            println!("{}", v.into_iter().collect::<String>());
        }
        if lower_bound / 495 != higher_bound / 495 {
            let target = (higher_bound / 495) * 495 - lower_bound;
            println!("{}", target);
            break;
        }
        upper_helper *= 10;
        append_count += 1;
    }
    return;
}
