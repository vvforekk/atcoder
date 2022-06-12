//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let mut b = vec![0; n];
    for i in 0..k {
        let mut c = vec![];
        for j in 0.. {
            let x = j * k + i;
            if x < n {
                c.push(a[x]);
            } else {
                break;
            }
        }

        for (j, e) in c.into_iter().sorted().enumerate() {
            b[j * k + i] = e;
        }
    }

    // dbg!(&b);

    let mut c = b.clone();
    c.sort();
    Yn(b == c);
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
