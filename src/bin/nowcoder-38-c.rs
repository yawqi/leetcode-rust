fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = nums[0];
    let mut k = nums[1];

    let mut counts = vec![];
    for time in (0..=(n + 1) / 2).rev() {
        if k == 0 {
            break;
        }
        if time * (time - 1) / 2 > k {
            continue;
        }
        while k >= time * (time - 1) / 2 {
            k -= time * (time - 1) / 2;
            counts.push(time);
        }
    }

    let mut ch = 'a';
    let mut chs = vec![];
    let mut left = n;
    for count in counts {
        let mut curr = vec![ch; count];
        chs.append(&mut curr);
        ch = ('a' as u8 + (ch as u8 + 1u8 - 'a' as u8) % 26u8) as char;
        left -= count;
        if left == 0 {
            break;
        }
        left -= 1;
        chs.push(ch);
        ch = ('a' as u8 + (ch as u8 + 1u8 - 'a' as u8) % 26u8) as char;
    }
    while left > 0 {
        chs.push(ch);
        ch = ('a' as u8 + (ch as u8 + 1u8 - 'a' as u8) % 26u8) as char;
        left -= 1;
    }

    println!("{}", chs.into_iter().collect::<String>());
}
