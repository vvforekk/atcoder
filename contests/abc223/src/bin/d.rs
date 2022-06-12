//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, m: usize,
        e: [(Usize1, Usize1); m]
    }

    let mut g = vec![Vec::new(); n];
    let mut d_in = vec![0usize; n];
    for (a, b) in e {
        g[a].push(b);
        d_in[b] += 1;
    }

    let mut s = (0..n)
        .filter(|&x| d_in[x] == 0)
        .map(|x| Reverse(x))
        .collect::<BinaryHeap<_>>();
    let mut ans = Vec::new();

    while let Some(Reverse(cur)) = s.pop() {
        ans.push(cur + 1);
        for &next in g[cur].iter() {
            d_in[next] -= 1;
            if d_in[next] == 0 {
                s.push(Reverse(next));
            }
        }
    }

    if d_in.into_iter().all(|x| x == 0) {
        vis!(ans);
    } else {
        vis!(-1);
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
    process::exit,
};
