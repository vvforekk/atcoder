//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, d: i64,
        a: [i64; n]
    }

    let s = a
        .iter()
        .enumerate()
        .map(|(i, x)| ((x + i as i64 * d) as i64, i))
        .collect_vec();
    let t = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (x - i as i64 * d, i))
        .collect_vec();
    let sts = SegmentTree::<Max<_>>::from(s);
    let stt = SegmentTree::<Max<_>>::from(t);

    let mut edges = Vec::new();
    for i in 1..n - 1 {
        let l = sts.range(..i);
        let r = stt.range(i + 1..);

    }

    let mst = kuruskal(n, &edges);
    let ans = mst.into_iter().map(|(_, _, c)| c).sum::<usize>();
    vis!(ans);
}

use sail::{
    graph::kruskal::kuruskal,
    prelude::*,
    segment_tree::{segment_tree::SegmentTree, Max},
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
    str::FromStr,
    string::ToString,
    usize::MAX,
};
