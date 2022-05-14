//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut dp = vec![vec![0usize; 2]; n];
    dp[0] = vec![a[n - 1], a[n - 1]];
    for i in 0..n - 1 {
        dp[i + 1][0] = dp[i][1];
        dp[i + 1][1] = min(dp[i][0], dp[i][1]) + a[i];
    }
    let x = min(dp[n - 1][0], dp[n - 1][1]);


    let mut a = a;
    a.rotate_left(1);
    let mut dp = vec![vec![0usize; 2]; n];
    dp[0] = vec![a[n - 1], a[n - 1]];
    for i in 0..n - 1 {
        dp[i + 1][0] = dp[i][1];
        dp[i + 1][1] = min(dp[i][0], dp[i][1]) + a[i];
    }
    let y = min(dp[n - 1][0], dp[n - 1][1]);

    let ans = min(x, y);
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
};
