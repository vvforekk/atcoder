//! This code is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports, clippy::needless_range_loop)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [usize; n]
    }

    let mut hate = vec![0usize; n];
    for i in 0..n {
        hate[x[i]] += c[i];
    }

    let mut q = PriorityQueue::new();
    for (i, &hate) in hate.iter().enumerate() {
        q.push(i, Reverse(hate));
    }

    let mut t = 0usize;
    while let Some((i, Reverse(hate))) = q.pop() {
        t += hate;
        q.change_priority_by(&x[i], |old| old.0 -= c[i]);
    }

    let ans = t;
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
use priority_queue::PriorityQueue;
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
