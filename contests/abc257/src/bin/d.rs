#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        p: [(i64, i64, i64); n]
    }

    let dist = |a: (i64, i64), b: (i64, i64)| (a.0 - b.0).abs() + (a.1 - b.1).abs();
    let mut d = vec![vec![0i64; n]; n];
    for i in 0..n {
        for j in 0..n {
            d[i][j] = (dist((p[i].0, p[i].1), (p[j].0, p[j].1)) + p[i].2 - 1) / p[i].2;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if max(d[i][k], d[k][j]) < d[i][j] {
                    d[i][j] = max(d[i][k], d[k][j]);
                }
            }
        }
    }

    // dbg!(&d);

    let ans = (0..n)
        .map(|from| d[from].iter().max().unwrap())
        .min()
        .unwrap();
    vis!(ans);
}

use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use ac_library_rs::math::{crt, inv_mod, pow_mod};
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num::{
    bigint::{BigInt, BigUint, ToBigInt as _, ToBigUint as _},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::{Ratio, Rational64},
    traits::{One, Pow, Zero},
};
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use rand::{
    random,
    rngs::{SmallRng, ThreadRng},
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng, SeedableRng,
};
use roaring::{RoaringBitmap, RoaringTreemap};
use sail::prelude::*;
use superslice::{Ext as _, Ext2 as _};
