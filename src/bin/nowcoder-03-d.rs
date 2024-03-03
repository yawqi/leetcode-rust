use std::io::Read;

fn main() {
    let mut n = String::new();
    let mut nums = String::new();
    std::io::stdin().read_line(&mut n);
    std::io::stdin().read_line(&mut nums);

    let mut nums = nums
        .trim()
        .split(' ')
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let n = nums.len();
    let mut visited = vec![false; n + 1];
    let mut indices = vec![];
    let mut values = vec![];

    for idx in 0..n {
        let num = nums[idx];
        if num > n as i32 || num < 1 || visited[num as usize] {
            indices.push(idx as usize + 1);
            continue;
        }

        visited[num as usize] = true;
    }

    for idx in 1..=n {
        if !visited[idx] {
            values.push(idx);
        }
    }

    indices
        .into_iter()
        .zip(values.into_iter())
        .for_each(|(i, v)| print!("{} {} ", i, v));
}
