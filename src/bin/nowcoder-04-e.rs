fn main() {
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums);
    let nums = nums
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let row = nums[0];
    let col = nums[1];
    let mut maze = vec![vec!['0'; col]; row];
    let mut is_a = true;
    if row >= col {
    for r in 0..row {
        let rev = r % 2 == 0;
        let its = if rev { (r..row) } else { (r..row).rev() };
        for idx in its {
            maze[r][idx] = if is_a { 'a' } else { 'b' };
            is_a = !is_a;
        }
    }
    } else {

    }
    for r in 
}
