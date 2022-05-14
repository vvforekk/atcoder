//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        e: [(Usize1, Usize1); m]
    }

    let mut g = vec![Vec::new(); n];
    for (u, v) in e {
        g[u].push(v);
        g[v].push(u);
    }
    let g = g;

    let mut visited = vec![false; n];
    let mut t1 = vec![];
    dfs(&g, &mut visited, &mut t1, 0, 0);

    for (a, b) in t1 {
        vis!((a + 1, b + 1));
    }

    let mut t2 = vec![];
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(cur) = q.pop_front() {
        for &next in g[cur].iter().filter(|&&x| !visited[x]).collect_vec() {
            q.push_back(next);
            t2.push((cur, next));
            visited[next] = true;
        }
    }

    for (a, b) in t2 {
        vis!((a + 1, b + 1));
    }
}

fn dfs(
    g: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    t: &mut Vec<(usize, usize)>,
    cur: usize,
    prev: usize,
) {
    if visited[cur] {
        return;
    } else {
        visited[cur] = true;
        if prev != cur {
            t.push((prev, cur));
        }
        for &next in g[cur].iter().filter(|&&x| !visited[x]).collect_vec() {
            dfs(g, visited, t, next, cur);
        }
    }
}

use sail::prelude::*;

use im_rc::{ordmap, ordset, OrdMap, OrdSet};
use itertools::{iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt, ToBigUint},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::Rational64,
    traits::{abs, abs_sub, Bounded, One, Pow, Saturating, Zero},
};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{auto::AutoSource, line::LineSource, once::OnceSource},
};
use rand::{
    random,
    rngs::SmallRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
};
