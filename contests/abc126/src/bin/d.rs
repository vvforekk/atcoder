//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1, i64); n - 1]
    }

    let mut g = vec![Vec::new(); n];
    for (a, b, c) in e {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut visited = vec![false; n];
    visited[0] = true;

    let mut s = vec![0];

    while let Some(cur) = s.pop() {
        for &(next, w) in g[cur].iter().filter(|x| !visited[x.0]).collect_vec() {
            dist[next] = dist[cur] + w;
            visited[next] = true;
            s.push(next);
        }
    }

    let ans = dist.into_iter().map(|x| x.is_even() as u8).collect_vec();
    vis!(ans;);
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
    usize::MAX,
};
