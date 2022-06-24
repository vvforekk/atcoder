//! This code is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports, clippy::needless_range_loop)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize,
        a: [(usize, usize); n]
    }

    let mut ans = vec![];
    let mut cl = 0usize;
    let mut cr = 0usize;
    for &(l, r) in a.iter().sorted() {
        if cr < l {
            if cl != 0 {
                ans.push((cl, cr));
            }
            cl = l;
            cr = r;
        } else {
            cr = max(cr, r);
        }
    }

    ans.push((cl, cr));

    for lr in ans {
        vis!(lr);
    }
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
