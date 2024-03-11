const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
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
    let begs = [['a', 'b', 'b'], ['a', 'c', 'c'], ['b', 'c', 'a']];
    for r in 0..begs.len() {
        for c in 0..begs[0].len() {
            maze[r][c] = begs[r][c];
        }
    }

    let choices = vec!['a', 'b', 'c', 'd'];
    for r in 0..row {
        for c in 0..col {
            if maze[r][c] == '0' {
                let mut found = vec![false; choices.len()];
                for dir in &DIRS {
                    let nr = r as i64 + dir.0;
                    let nc = c as i64 + dir.1;
                    if nr >= 0 && nc >= 0 && nr < row as i64 && nc < col as i64 {
                        let ch = maze[nr as usize][nc as usize];
                        if ch == '0' {
                            continue;
                        }
                        let idx = ch as usize - 'a' as usize;
                        found[idx] = true;
                    }
                }
                let idx = found
                    .iter()
                    .enumerate()
                    .find(|(idx, v)| **v == false)
                    .unwrap()
                    .0;
                maze[r][c] = ('a' as u8 + idx as u8) as char;
            }
        }
    }
    for r in 0..row {
        for c in 0..col {
            print!("{}", maze[r][c]);
        }
        println!("");
    }
}
