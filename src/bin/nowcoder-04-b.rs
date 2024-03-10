fn main() {
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums);
    let nums = nums
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut sum = 0;
    let mut row_xors = vec![0; nums[0]];
    let mut col_xors = vec![0; nums[1]];
    for r in 0..nums[0] {
        let mut row = String::new();
        std::io::stdin().read_line(&mut row);
        let mut c = 0;
        row.trim().split(' ').for_each(|v| {
            let v = v.parse::<usize>().unwrap();
            sum += v;
            col_xors[c] ^= v;
            row_xors[r] ^= v;
            c += 1;
        });
    }
    if sum != nums[2] {
        println!("wrong answer");
        return;
    }
    let xor = row_xors[0];
    if row_xors
        .into_iter()
        .chain(col_xors.into_iter())
        .all(|v| v == xor)
    {
        println!("accepted");
    } else {
        println!("wrong answer");
    }
}
