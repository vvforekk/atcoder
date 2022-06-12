//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        i: [Usize1; n],
    }

    let mut ans = vec![(0usize, 0usize); n];
    let mut h = vec![0usize; n];

    let mut ii = 0usize;
    for root in p {
        let mut prev = root;
        while i[ii] != root {
            ans[prev].0 = i[ii];
            prev = i[ii];
            ii += 1;
        }
    }
}

use sail::prelude::*;
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
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
