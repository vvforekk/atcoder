//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

fn main() {
    input! {
        n: usize
    }

    let mut ans = 0usize;
    for i in 1..=n {
        let mut k = i;
        for d in (2..).map(|x| x * x) {
            if k < d {
                break;
            }
            while k % d == 0 {
                k /= d;
            }
        }
        ans += (1..).map(|x| x * x).take_while(|&x| k * x <= n).count();
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
use sail::{
    prelude::*,
    prime::sieve::{atkin::SieveOfAtkin, PrimeSieve},
};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};
