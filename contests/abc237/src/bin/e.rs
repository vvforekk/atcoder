//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        h: [i64; n],
        e: [(Usize1, Usize1); m]
    }

    let mut g = DWLGraph::<u64>::new(n);
    for (i, j) in e {
        g.add_edge(
            i,
            j,
            if h[i] >= h[j] {
                0
            } else {
                (h[j] - h[i]) as u64
            },
        );
        g.add_edge(
            j,
            i,
            if h[j] >= h[i] {
                0
            } else {
                (h[i] - h[j]) as u64
            },
        );
    }

    let d = g
        .dijkstra(0)
        .into_iter()
        .enumerate()
        .map(|(i, d)| d.unwrap() as i64 + (h[i] - h[0]))
        .min()
        .unwrap()
        .neg();
    vis!(d);
}

use sail::{
    graph::{bellman_ford::bellman_ford, dijkstra::dijkstra, DWLGraph},
    prelude::*,
};

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
    ops::Neg,
    str::FromStr,
    string::ToString,
    usize::MAX,
};
