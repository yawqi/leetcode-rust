fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut x = nums[0];
    let mut y = nums[1];
    let l = nums[2];
    let r = nums[3];
    if y > x {
        let tmp = x;
        x = y;
        y = tmp;
    }
    let d = gcd(x, y);
    x /= d;
    y /= d;
    let upperbound = r / x;
    let lowerbound = (l + y - 1) / y;
    let count = if upperbound >= lowerbound {
        upperbound - lowerbound + 1
    } else {
        0
    };
    println!("{}", count);
}
