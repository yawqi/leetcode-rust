fn main() {
    let mut s1 = String::new();
    let mut s2 = String::new();
    std::io::stdin().read_line(&mut s1);
    std::io::stdin().read_line(&mut s2);
    let nm = s1
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let lens = s2
        .trim()
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let n = nm[0];
    let m = nm[1];

    let mut depths = vec![vec![]; n];
    depths[0].push(1);
    if n > m + 1 || m > n * (n - 1) / 2 {
        println!("-1");
        return;
    }

    let mut max_depth = 0;
    lens.iter().enumerate().skip(1).for_each(|(idx, len)| {
        depths[*len].push(idx + 1);
        max_depth = max_depth.max(*len);
    });

    let mut prev_vertex = 1;
    let mut graph = vec![vec![]; n + 1];
    let mut curr_edges = 0;
    for depth in 1..=max_depth {
        if depths[depth].is_empty() {
            println!("-1");
            return;
        }

        for v in &depths[depth] {
            graph[prev_vertex].push(*v);
            curr_edges += 1;
        }
        prev_vertex = depths[depth][0];
    }

    if curr_edges < m {
        'outer: for depth in &depths {
            for i in 0..depth.len() {
                for j in i + 1..depth.len() {
                    graph[depth[i]].push(depth[j]);
                    curr_edges += 1;
                    if curr_edges == m {
                        break 'outer;
                    }
                }
            }
        }
    }

    if curr_edges < m {
        'outer: for i in 1..max_depth {
            let j = i + 1;
            if depths[i].len() == 1 {
                continue;
            }

            for from in &depths[i][1..] {
                for to in &depths[j] {
                    graph[*from].push(*to);
                    curr_edges += 1;
                    if curr_edges == m {
                        break 'outer;
                    }
                }
            }
        }
    }

    if curr_edges < m {
        println!("-1");
        return;
    }

    for (from, tos) in graph.into_iter().enumerate() {
        for to in tos {
            println!("{} {}", from, to);
        }
    }
}
