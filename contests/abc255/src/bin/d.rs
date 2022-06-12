//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [i64; n]
    }

    a.sort_unstable();
    let acc = Accumulation::from(a.clone());

    for _ in 0..q {
        input! {
            x: i64,
        }

        let d = a.upper_bound(&x);
        let ans =
            x * d as i64 - acc.range_sum(0..d) + acc.range_sum(d..n) - x * (n as i64 - d as i64);
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
use sail::{accumulate::Accumulation, prelude::*};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
use superslice::Ext;
