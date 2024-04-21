struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut visited = vec![false; n];
        let mut dists = vec![i32::MAX; n];
        let mut graph = vec![vec![]; n];
        let mut ret = vec![false; edges.len()];
        for idx in 0..edges.len() {
            let edge = &edges[idx];
            let v1 = edge[0] as usize;
            let v2 = edge[1] as usize;
            let len = edge[2];
            graph[v1].push((v2, idx));
            graph[v2].push((v1, idx));
        }

        let mut bh = BinaryHeap::new();
        bh.push(Reverse((0, 0usize)));
        while !bh.is_empty() {
            let Reverse((dist, curr)) = bh.pop().unwrap();
            if visited[curr] {
                continue;
            }
            dists[curr] = dist;
            visited[curr] = true;
            for &(nxt_v, e_idx) in &graph[curr] {
                if visited[nxt_v] {
                    continue;
                }
                bh.push(Reverse((dist + edges[e_idx as usize][2], nxt_v)));
            }
        }

        let mut q = VecDeque::new();
        if dists[n - 1] != -1 {
            q.push_back(n - 1);
        }

        while !q.is_empty() {
            let curr = q.pop_front().unwrap();
            for &(prev_v, e_idx) in &graph[curr] {
                if edges[e_idx][2] + dists[prev_v] == dists[curr] {
                    ret[e_idx] = true;
                    q.push_back(prev_v);
                }
            }
        }
        ret
    }
}

fn main() {
    let v = vec![
        vec![3, 0, 6],
        vec![1, 3, 6],
        vec![2, 1, 8],
        vec![5, 2, 1],
        vec![4, 2, 1],
        vec![3, 5, 9],
        vec![5, 1, 5],
        vec![0, 4, 10],
    ];
    let res = Solution::find_answer(6, v);
}
