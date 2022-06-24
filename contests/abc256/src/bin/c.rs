//! This code is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports, clippy::needless_range_loop)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        h: [usize; 3],
        w: [usize; 3],
    }

    let mut a = vec![Vec::new(); 3];
    for l in 0..3 {
        for i in 1..=h[l] - 2 {
            for j in 1..=h[l] - i - 1 {
                let k = h[l] - i - j;
                a[l].push([i, j, k]);
            }
        }
    }


    let mut ans = 0usize;
    for ([a1, a2, a3], [b1, b2, b3], [c1, c2, c3]) in
        iproduct!(a[0].iter(), a[1].iter(), a[2].iter())
    {
        if a1 + b1 + c1 == w[0] && a2 + b2 + c2 == w[1] && a3 + b3 + c3 == w[2] {
            ans += 1;
        }
    }

    vis!(ans);
}

use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt, ToBigUint},
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
