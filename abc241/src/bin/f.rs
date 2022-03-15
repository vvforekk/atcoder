use std::collections::{BTreeSet, VecDeque};

use proconio::marker::Usize1;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        n: usize,
        start: (Usize1, Usize1),
        goal: (Usize1, Usize1),
        oo: [(Usize1, Usize1); n]
    }

    let mut g = BTreeSet::new();
    for obj in oo {
        g.insert(obj);
    }

    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some(((i, j), cost)) = q.pop_front() {
        if (i, j) == goal {
            println!("{}", cost);
            return;
        }

        for i in (1..i).rev() {
            if g.contains(&(i - 1, j)) {
                q.push_back(((i, j), cost + 1));
            }
        }
        for i in (i + 1 .. h - 1).rev() {
            if g.contains(&(i - 1, j)) {
                q.push_back(((i, j), cost + 1));
            }
        }
        for i in (1..i).rev() {
            if g.contains(&(i - 1, j)) {
                q.push_back(((i, j), cost + 1));
            }
        }
        for i in (1..i).rev() {
            if g.contains(&(i - 1, j)) {
                q.push_back(((i, j), cost + 1));
            }
        }
    }

    println!("-1");
}
