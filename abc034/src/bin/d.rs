//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        c: [(f64, f64); n]
    }

    let mut dp = vec![vec![None; k + 1]; n + 1];

    for i in 0..n {
        for j in 0..=k {
            let mut p = vec![];
            if let Some((m, w)) = dp[i][j] {
                p.push((m, w));
            }

            let cm = c[i].0 * c[i].1 / 100.;
            let cw = c[i].0;
            match dp[i][j] {
                Some((m, w)) if j < k => {
                    if (m + cm) / (w + cw) > cm / cw {
                        dp[i + 1][j + 1] = Some(((m + cm), (w + cw)));
                    } else {
                        dp[i + 1][j] = dp[i][j];
                    }
                }
                None if j < k => dp[i + 1][j + 1] = Some((cm, cw)),
                _ => {}
            }
        }
    }

    dbg!(&dp);
}

use ordered_float::OrderedFloat;
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
