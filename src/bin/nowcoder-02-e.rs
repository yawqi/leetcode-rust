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
    let mut reds = vec![0; n];
    let mut whites = vec![0; n];
    dfs(0, 0, &edges, &mut reds, &mut whites);
    (reds[0] + whites[0]) % 1000000007
}

fn dfs(
    node: usize,
    parent: usize,
    edges: &Vec<Vec<usize>>,
    reds: &mut Vec<usize>,
    whites: &mut Vec<usize>,
) {
    let mut ret_r = 1;
    let mut ret_w = 1;

    for nxt_node in &edges[node] {
        if *nxt_node == parent {
            continue;
        }
        dfs(*nxt_node, node, edges, reds, whites);
        ret_r = (reds[*nxt_node] + whites[*nxt_node]) % 1000000007 * ret_r % 1000000007;
        ret_w = reds[*nxt_node] * ret_w % 1000000007;
    }
    reds[node] = ret_r;
    whites[node] = ret_w;
}
