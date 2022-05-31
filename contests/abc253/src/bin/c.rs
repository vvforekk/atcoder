//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        q: usize,
    }

    let mut set = BTreeMap::new();

    for _ in 0..q {
        input! {
            t: usize
        }

        if t == 1 {
            input! {
                x: usize,
            }

            *set.entry(x).or_insert(0) += 1;
        } else if t == 2 {
            input! {
                x: usize, c: usize,
            }

            if let Some(h) = set.get_mut(&x) {
                *h -= min(c, *h);
                if *h == 0 {
                    set.remove(&x);
                }
            }
        } else {
            let ans = set.iter().next_back().unwrap().0 - set.iter().next().unwrap().0;
            vis!(ans);
        }
    }
}

// use itertools::{iproduct, izip, Itertools as _};
// use itertools_num::ItertoolsNum as _;
// use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
// use num::{
//     bigint::{BigInt, BigUint, ToBigInt as _, ToBigUint as _},
//     complex::Complex64,
//     integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
//     rational::{Ratio, Rational64},
//     traits::{One, Pow, Zero},
// };
// use rand::{
//     random,
//     rngs::{SmallRng, ThreadRng},
//     seq::{IteratorRandom, SliceRandom},
//     thread_rng, Rng, SeedableRng,
// };
use sail::prelude::*;
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
