//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, k: usize,
        a: [Usize1; k],
        p: [(f64, f64); n]
    }

    let mut f = vec![false; n];
    for &i in a.iter() {
        f[i] = true;
    }

    let mut ans = 0.0;
    for i in 0..n {
        if !f[i] {
            let near = a
                .iter()
                .map(|&x| ((p[x].0 - p[i].0).powi(2) + (p[x].1 - p[i].1).powi(2)).sqrt())
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            if near > ans {
                ans = near;
            }
        }
    }

    vis!(ans);
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
    f64::INFINITY,
};
