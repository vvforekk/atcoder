//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        s: Chars
    }

    let s = s
        .into_iter()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .rev()
        .collect_vec();

    let mut ans = 0usize;
    let mut c = HashMap::new();
    c.insert(0, 1);
    let mut a = 0usize;
    for i in 0..s.len() {
        a = (a + s[i] * mod_pow(10, i, 2019)) % 2019;
        ans += c.get(&a).unwrap_or(&0);
        *c.entry(a).or_insert(0) += 1;
    }

    vis!(ans);
}

fn mod_pow(a: usize, b: usize, p: usize) -> usize {
    let mut res = 1usize;
    let mut m = a;
    for i in (0..).take_while(|&x| 1 << x <= b) {
        if b & (1 << i) != 0 {
            res *= m;
            res %= p;
        }
        m *= m;
        m %= p;
    }
    res
}

#[test]
fn mp() {
    assert_eq!(mod_pow(10, 0, 100000), 1);
    assert_eq!(mod_pow(10, 1, 100000), 10);
    assert_eq!(mod_pow(10, 2, 100000), 100);
    assert_eq!(mod_pow(10, 3, 100000), 1000);
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
    usize::MAX,
};
