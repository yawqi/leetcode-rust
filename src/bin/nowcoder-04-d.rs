use std::collections::VecDeque;
const DIRS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
fn main() {
    let mut nums = String::new();
    std::io::stdin().read_line(&mut nums);
    let nums = nums
        .trim()
        .split(' ')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let row = nums[0];
    let col = nums[1];

    let mut maze = vec![];
    for i in 0..row {
        let mut curr_row = String::new();
        std::io::stdin().read_line(&mut curr_row);
        let curr_row = curr_row.trim().chars().collect::<Vec<_>>();
        maze.push(curr_row);
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; col]; row];
    visited[0][0] = true;
    queue.push_back((0, 0));

    let mut curr_steps = 0;
    while !queue.is_empty() {
        let sz = queue.len();
        for _ in 0..sz {
            let (crow, ccol) = queue.pop_front().unwrap();
            let cur = maze[crow][ccol];
            if crow == row - 1 && ccol == col - 1 {
                println!("{}", curr_steps);
                return;
            }
            for dir in &DIRS {
                let nrow = crow as i64 + dir.0;
                let ncol = ccol as i64 + dir.1;
                if nrow >= 0
                    && nrow < maze.len() as i64
                    && ncol >= 0
                    && ncol < maze[0].len() as i64
                    && maze[nrow as usize][ncol as usize] != cur
                    && !visited[nrow as usize][ncol as usize]
                {
                    visited[nrow][ncol] = true;
                    queue.push_back((nrow as usize, ncol as usize));
                }
            }
        }
        curr_steps += 1;
    }

    println!("-1");
}
