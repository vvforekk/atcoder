//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, w: usize,
        a: [(usize, usize); n]
    }

    let v = n * 1000;
    let mut dp = vec![vec![MAX; v + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 0;
    }

    for (i, &(wi, vi)) in a.iter().enumerate() {
        for j in 0..v {
            if dp[i][j] != MAX {
                dp[i + 1][j + vi] = min(dp[i + 1][j + vi], dp[i][j] + wi);
                dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
            }
        }
    }

    let ans = (0..=v).filter(|&x| dp[n][x] <= w).max().unwrap();
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
