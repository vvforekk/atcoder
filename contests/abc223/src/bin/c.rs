//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize,
        a: [(f64, f64); n]
    }

    let c = a.iter().map(|&(x, y)| x / y).collect_vec();
    let time = Accumulation::from(c).accumulation().to_owned();
    let mut ans = 0.0;
    let all = time[n];

    for i in 0..n {
        if time[i + 1] > all / 2.0 {
            let d = all / 2.0 - time[i];
            ans += a[i].1 * d;
            break;
        } else {
            ans += a[i].0;
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
use sail::{accumulate::Accumulation, prelude::*};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::EPSILON,
};
