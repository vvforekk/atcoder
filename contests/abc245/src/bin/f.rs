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

    let mut scc = SccGraph::new(n);
    for (u, v) in e {
        g[v].push(u);
        scc.add_edge(u, v);
    }

    let mut ans = vec![false; n];
    scc.scc()
        .into_iter()
        .filter(|x| x.len() >= 2)
        .for_each(|x| {
            let p = x[0];
            let mut visited = vec![false; n];
            let mut s = vec![p];
            while let Some(cur) = s.pop() {
                visited[cur] = true;
                ans[cur] = true;
                for &next in g[cur].iter().filter(|&&x| !visited[x]) {
                    s.push(next);
                }
            }
        });

    vis!(ans.into_iter().filter(|&x| x).count());
}

use ac_library_rs::SccGraph;
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
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
    usize::MAX,
};
