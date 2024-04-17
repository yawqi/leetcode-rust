fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s);
    let mut nums = s
        .trim()
        .split(' ')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = nums[0];
    let m = nums[1];
    let mut maze = vec![];
    let mut dir = (0i32, 0i32);
    let mut pos = (0, 0);
    for r in 0..n {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s);
        let line = s.trim().chars().collect::<Vec<_>>();
        for c in 0..line.len() {
            if line[c].is_ascii_alphabetic() {
                pos.0 = r;
                pos.1 = c;
                dir = match line[c] {
                    'W' => (0, -1),
                    'S' => (0, 1),
                    'A' => (-1, 0),
                    'D' => (1, 0),
                    _ => (0, 0),
                };
            }
        }
        maze.push(line);
    }

    let mut x = pos.0;
    let mut y = pos.1;
    let mut count = 0;
    while x > 0 && x <= n && y > 0 && y <= m {
        if maze[x - 1][y - 1] == '*' {
            count += 1;
        }
        x = (x as i32 + dir.0) as usize;
        y = (y as i32 + dir.1) as usize;
    }
    println!("{}", count);
}
