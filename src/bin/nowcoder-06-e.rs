use std::collections::VecDeque;
const DIRS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let t = nums[0];
    let mut ans = vec![];
    for _ in 0..t {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let nums = s
            .trim()
            .split(' ')
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = nums[0];
        let m = nums[1];
        let mut maze = vec![];
        for _ in 0..n {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s);
            let nums = s.trim().chars().collect::<Vec<_>>();
            maze.push(nums);
        }
        let mut s_pos = (0, 0);
        let mut t_pos = (0, 0);
        for r in 0..n {
            for c in 0..m {
                if maze[r][c] == 'S' {
                    s_pos = (r, c);
                } else if maze[r][c] == 'T' {
                    t_pos = (r, c);
                }
            }
        }

        let mut q = VecDeque::new();
        let mut steps = 0;
        let mut visited = vec![vec![vec![false; 4]; m]; n];
        for d in 0..4 {
            q.push_back((s_pos.0, s_pos.1, d));
        }
        let mut found = false;
        'outer: while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let curr = q.pop_front().unwrap();
                let curr_r = curr.0;
                let curr_c = curr.1;
                let d = curr.2;
                if visited[curr_r][curr_c][d] {
                    continue;
                }
                visited[curr_r][curr_c][d] = true;
                if maze[curr_r][curr_c] == 'T' {
                    found = true;
                    break 'outer;
                } else if maze[curr_r][curr_c] == '*' {
                    for nxt_d in 0..4 {
                        if nxt_d == d || nxt_d % 2 != d % 2 {
                            let (nxt_r, nxt_c) = (DIRS[nxt_d].0, DIRS[nxt_d].1);
                            if (nxt_r + curr_r as i64) < n as i64
                                && (nxt_c + curr_c as i64) < m as i64
                                && (nxt_r + curr_r as i64) >= 0 as i64
                                && (nxt_c + curr_c as i64) >= 0 as i64
                            {
                                q.push_back((
                                    (nxt_r + curr_r as i64) as usize,
                                    (nxt_c + curr_c as i64) as usize,
                                    nxt_d,
                                ));
                            }
                        }
                    }
                } else if maze[curr_r][curr_c] == '.' {
                    let (nxt_r, nxt_c) = (DIRS[d].0, DIRS[d].1);
                    if (nxt_r + curr_r as i64) < n as i64
                        && (nxt_c + curr_c as i64) < m as i64
                        && (nxt_r + curr_r as i64) >= 0 as i64
                        && (nxt_c + curr_c as i64) >= 0 as i64
                    {
                        q.push_back((
                            (nxt_r + curr_r as i64) as usize,
                            (nxt_c + curr_c as i64) as usize,
                            d,
                        ));
                    }
                }
            }
            steps += 1;
        }
        if found {
            ans.push(steps);
        } else {
            ans.push(-1);
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
