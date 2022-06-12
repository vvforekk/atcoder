//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, m: usize,
        s: [i64; n - 1],
        x: [i64; m]
    }

    let set: HashSet<_> = x.iter().copied().collect();
    let mut ans = 0usize;
    for i in 0..n {
        for &x in x.iter() {
            let mut a = vec![i64::MAX; n];
            a[i] = x;
            for j in i + 1..n {
                a[j] = s[j - 1] - a[j - 1];
            }
            for j in (0..i).rev() {
                a[j] = s[j] - a[j + 1];
            }

            let c = a.iter().filter(|&x| set.contains(x)).count();
            ans = max(ans, c);
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
};
