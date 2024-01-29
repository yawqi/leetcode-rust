fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n);
    let n = n.trim().parse::<usize>().unwrap();
    let mut edges = vec![vec![]; n];
    let mut visited = vec![false; n];
    let mut s = String::new();
    for _ in 0..n - 1 {
        std::io::stdin().read_line(&mut s);
        let nums = s
            .trim()
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let u = nums[0];
        let v = nums[1];
        edges[u - 1].push(v - 1);
        edges[v - 1].push(u - 1);
        s.clear();
    }
    println!("{}", dfs(0, &edges, &mut visited, true) % (1000000007));
}

fn dfs(
    node: usize,
    edges: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    is_parent_red: bool,
) -> usize {
    let mut ret_r = 1;
    let mut ret_w = 1;
    visited[node] = true;
    for nxt_node in &edges[node] {
        if visited[*nxt_node] {
            continue;
        }
        ret_r *= dfs(*nxt_node, edges, visited, true);
        if is_parent_red {
            ret_w *= dfs(*nxt_node, edges, visited, false);
        }
    }
    visited[node] = false;
    if is_parent_red {
        ret_r + ret_w
    } else {
        ret_r
    }
}
