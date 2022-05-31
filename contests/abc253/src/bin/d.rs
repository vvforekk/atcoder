//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: i128, a: i128, b: i128,
    }

    let ab = lcm(a, b);
    let ans = n * (n + 1) / 2 + ab * (n / ab) * (n / ab + 1) / 2
        - a * (n / a) * (n / a + 1) / 2
        - b * (n / b) * (n / b + 1) / 2;
    vis!(ans);
}

fn t(n: i128, a: i128, b: i128) -> i128 {
    (1..=n).filter(|&x| x % a != 0 && x % b != 0).sum()
}

#[test]
fn test() {
    let n = 100;
    for _ in 0..1000 {
        let a = (random::<i128>() % n).abs();
        let b = (random::<i128>() % n).abs();

        let ab = a * b;
        let ans = n * (n + 1) / 2 + ab * (n / ab) * (n / ab + 1) / 2
            - a * (n / a) * (n / a + 1) / 2
            - b * (n / b) * (n / b + 1) / 2;

        if ans != t(n, a, b) {
            dbg!(ans, t(n, a, b), a, b);
        }
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
use sail::prelude::*;
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
