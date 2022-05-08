//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![]; 1 << (2 * n)]; // [cur][used]
    dp[0] = vec![0];
    for i in 0..2 * n - 1 {
        input! {
            a: [usize; 2 * n - i - 1]
        }

        for s in 0..1 << (2 * n) {
            for j in i + 1..2 * n {
                if s & (1 << i | 1 << j) == 0 {
                    for t in dp[s].iter().copied().collect_vec() {
                        dp[s | (1 << i | 1 << j)].push(t ^ a[j - i - 1]);
                    }
                }
            }
        }
    }

    // dbg!(&dp);

    let &ans = dp[(1 << (2 * n)) - 1].iter().max().unwrap();
    vis!(ans);
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
