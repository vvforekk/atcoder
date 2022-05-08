//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    }

    let g = DULGraph::from_edges(n, &edges);
    let mut l = vec![None; n];

    let ans = (0..n).map(|start| dfs(&g, &mut l, start)).max().unwrap();
    vis!(ans);
}

fn dfs(graph: &DULGraph, memo: &mut [Option<usize>], cur: usize) -> usize {
    if let Some(res) = memo[cur] {
        res
    } else {
        let adj = graph.adjacencies(cur);
        if adj.is_empty() {
            memo[cur] = Some(0);
            0
        } else {
            let res = graph
                .adjacencies(cur)
                .into_iter()
                .map(|next| dfs(graph, memo, next))
                .max()
                .unwrap()
                + 1;
            memo[cur] = Some(res);
            res
        }
    }
}

use sail::{graph::DULGraph, prelude::*};

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
    usize::MAX,
};
