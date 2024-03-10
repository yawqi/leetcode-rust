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
    let mut visitd = vec![vec![i32::MAX; col]; row];
    visitd[0][0] = 0;
    if let Some(step) = dfs(&maze, 0, 0, &mut visitd, 0) {
        println!("{}", step);
    } else {
        println!("-1");
    }
}

fn dfs(
    maze: &Vec<Vec<char>>,
    crow: usize,
    ccol: usize,
    visitd: &mut Vec<Vec<i32>>,
    steps: usize,
) -> Option<i32> {
    if crow == maze.len() - 1 && ccol == maze[0].len() - 1 {
        return Some(steps as i32);
    }
    let cur = maze[crow][ccol];
    let mut ret = i32::MAX;
    for dir in &DIRS {
        let nrow = crow as i64 + dir.0;
        let ncol = ccol as i64 + dir.1;
        if nrow >= 0
            && nrow < maze.len() as i64
            && ncol >= 0
            && ncol < maze[0].len() as i64
            && maze[nrow as usize][ncol as usize] != cur
        {
            if visitd[nrow as usize][ncol as usize] > steps as i32 + 1 {
                visitd[nrow as usize][ncol as usize] = steps as i32 + 1;
            }
        }
    }
    for dir in &DIRS {
        let nrow = crow as i64 + dir.0;
        let ncol = ccol as i64 + dir.1;
        if nrow >= 0
            && nrow < maze.len() as i64
            && ncol >= 0
            && ncol < maze[0].len() as i64
            && maze[nrow as usize][ncol as usize] != cur
        {
            if visitd[nrow as usize][ncol as usize] == steps as i32 + 1 {
                if let Some(v) = dfs(maze, nrow as usize, ncol as usize, visitd, steps + 1) {
                    ret = ret.min(v);
                    if ret as usize <= steps + maze.len() - crow + maze[0].len() - ccol - 1 {
                        return Some(ret);
                    }
                }
            }
        }
    }
    if ret == i32::MAX {
        None
    } else {
        Some(ret)
    }
}
