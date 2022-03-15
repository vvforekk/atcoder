use std::collections::{BinaryHeap, VecDeque};

use proconio::marker::Usize1;

const INF: usize = 1 << 20;

fn main() {
    proconio::input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    }

    let mut g = vec![Vec::new(); n];
    let mut c = vec![0; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
        c[a] += 1;
        c[b] += 1;
    }
    let g = g;
    let start = c.into_iter().filter(|&x| x == 1).next().unwrap();
    let mut d = vec![INF; n];
    d[start] = 0;
    let mut heap = VecDeque::new();
    for &one in g[start].iter() {
        heap.push_back((start, one));
        d[one] = 1;
    }
    while let Some((cur, cost)) = heap.pop_front() {
        let mut i = 0;
        for &next in g[cur].iter() {
            if cost + 1 < d[next] {
                let new_cost = if i == 0 { cost + 1 } else { 1 };
                d[next] = new_cost;
                heap.push_back((next, new_cost));
            }
        }
    }
}
