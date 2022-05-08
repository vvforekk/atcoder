//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        h: usize, w: usize,
        b: [Chars; h]
    }

    let mut dp = vec![vec![0; w]; h];

    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w - 1 {
            if b[i][j] == '.' && dp[i][j] > 0 {
                if i + 1 < h && b[i + 1][j] == '.' {
                    dp[i + 1][j] = max(dp[i][j] + 1, dp[i + 1][j]);
                }
                if b[i][j + 1] == '.' {
                    dp[i][j + 1] = max(dp[i][j] + 1, dp[i][j + 1]);
                }
            }
        }
    }

    let ans = dp.into_iter().flatten().max().unwrap();
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
