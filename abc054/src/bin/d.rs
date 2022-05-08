//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, a: usize, b: usize,
        p: [(usize, usize, i64); n]
    }

    let mut dp = vec![vec![(INF, 0usize); n * 10 + 1]; n * 10 + 1];
    dp[0][0] = (0, 0);
    for i in 0..n * 10 {
        for j in 0..n * 10 {
            for (t, &(a, b, c)) in p.iter().enumerate() {
                if i + a <= n * 10 && j + b <= n * 10 && dp[i][j].1 & (1 << t) == 0 {
                    dp[i + a][j + b] =
                        min(dp[i + a][j + b], (dp[i][j].0 + c, dp[i][j].1 | (1 << t)));
                }
            }
        }
    }

    // dbg!(&dp);

    let ans = (1..)
        .map(|m| (a * m, b * m))
        .take_while(|&(a, b)| a <= n * 10 && b <= n * 10)
        .map(|x| dp[x.0][x.1].0)
        .filter(|&x| x < INF)
        .min()
        .unwrap_or(-1);
    vis!(ans)
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
