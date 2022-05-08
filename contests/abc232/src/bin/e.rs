//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        h: usize, w: usize, k: usize,
        (a, b): (Usize1, Usize1),
        (x, y): (Usize1, Usize1),
    }

    let mut dp = vec![vec![ModInt998244353::zero(); 4]; k + 1];
    dp[0][0] = 1.into();

    for i in 0..k {
        dp[i + 1][0] = dp[i][1] * (h - 1) + dp[i][2] * (w - 1);
        dp[i + 1][1] = dp[i][0] + dp[i][1] * (h - 2) + dp[i][3] * (w - 1);
        dp[i + 1][2] = dp[i][0] + dp[i][2] * (w - 2) + dp[i][3] * (h - 1);
        dp[i + 1][3] = dp[i][1] + dp[i][2] + dp[i][3] * (h + w - 4);
    }

    let ans = dp[k][if a == x && b == y {
        0
    } else if b == y {
        1
    } else if a == x {
        2
    } else {
        3
    }];
    vis!(ans.get());
}

use ac_library_rs::Mod998244353;
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
