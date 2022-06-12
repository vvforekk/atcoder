//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports, clippy::needless_range_loop)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, m: usize,
        e: [(Usize1, Usize1); m],
        q: usize,
        qs: [(Usize1, usize); q]
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in e {
        g[a].push(b);
        g[b].push(a);
    }

    for (x, k) in qs {
        let mut q = VecDeque::new();
        q.push_back((x, 0));

        let mut gr = vec![];
        while let Some((cur, dist)) = q.pop_front() {
            gr.push(cur);
            for &next in g[cur].iter() {
                if dist < k {
                    q.push_back((next, dist + 1));
                }
            }
        }

        let ans: usize = gr.into_iter().unique().map(|x| x + 1).sum();
        vis!(ans);
    }
}

use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt as _, ToBigUint as _},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::{Ratio, Rational64},
    traits::{One, Pow, Zero},
};
use rand::{
    random,
    rngs::{SmallRng, ThreadRng},
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng, SeedableRng,
};
use sail::prelude::*;
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
