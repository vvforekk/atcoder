//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, k: usize,
    }

    let mut used = vec![false; n + 1];
    let mut c = 0usize;
    for i in (1..=n).rev() {
        if !used[i] {
            c += 1;
            for d in (1..).take_while(|&x| x * x <= n).filter(|&x| i % x == 0) {
                used[d] = true;
                used[i / d] = true;
            }
        }
    }

    if k >= c {
        let ans = ModInt998244353::new(n - c).combination(ModInt998244353::new(k - c));
        vis!(ans.get());
    } else {
        vis!(0);
    }
}

use sail::{
    modint::factorial::Factorial,
    prelude::*,
    prime::sieve::{atkin::SieveOfAtkin, PrimeSieve},
};

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
