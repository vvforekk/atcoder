//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: u128, k: u128, m: u128,
    }

    // m ^ k ^ n
    let p = 998244353;

    // let phi = n % (1..p - 1).filter(|x| (p - 1).gcd(x) == 1).count() as u128;
    // let phi = n % 402653184;

    let kn = if !k.is_multiple_of(&(p - 1)) {
        pow_mod(k, n, p - 1)
    } else {
        0
    };
    let ans = if !m.is_multiple_of(&p) {
        pow_mod(m, kn, p)
    } else {
        0
    };
    vis!(ans);
}

fn pow_mod(n: u128, mut m: u128, p: u128) -> u128 {
    let mut res = 1;
    let mut cur = n % p;
    while m > 0 {
        if m.is_odd() {
            res *= cur;
            res %= p;
        }
        cur *= cur;
        cur %= p;
        m >>= 1;
    }
    res
}

#[test]
fn phi() {
    let p = 998244353 - 1;
    let mut c = 0;
    for i in 1..p {
        if p.gcd(&i) == 1 {
            c += 1;
        }
    }

    dbg!(c);
}

use sail::prelude::*;

use im_rc::{ordmap, ordset, OrdMap, OrdSet};
use itertools::{iproduct, izip, Itertools};
use itertools_num::ItertoolsNum as _;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt, ToBigUint},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::Rational64,
    traits::{abs, abs_sub, Bounded, One, Pow, Saturating, Zero},
};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{auto::AutoSource, line::LineSource, once::OnceSource},
};
use rand::{
    random,
    rngs::SmallRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
};
