//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // (i, parity, trailing, prev);
}

fn rec(s: &[char], i: usize, parity: bool, trailing: u8, prev: Option<usize>) {
    match s[i] {
        'A' => {
            rec(s, i + 1, parity, 1, prev);
        }
        'R' => {}
        'C' => {}
        _ => unreachable!(),
    }
}

// let mut dp = vec![vec![vec![0usize; 3]; 2]; n + 1]; // i回の操作後 直前の操作の偶奇 直前は'', 'A', 'AR'
// dp[0][0][0] = 0;

// for i in 0..n {
//     for parity in 0..2 {
//         match s[i] {
//             'A' => {
//                 // dp[i + 1][parity][0] += 0;
//                 // dp[i + 1][parity][1] += dp[i][parity][0] + dp[i][parity][1] + dp[i][parity][2];
//                 // dp[i + 1][parity][2] += 0;
//             }
//             'R' => {
//                 // dp[i + 1][parity][0] += dp[i][parity][0] + dp[i][parity][2];
//                 // dp[i + 1][parity][1] += 0;
//                 // dp[i + 1][parity][2] += dp[i][parity][1];
//             }
//             'C' => {
//                 if parity.is_even() {
//                     // R
//                     if dp[i - 2][parity][1] > 0 {
//                         // dp[i + 1][parity ^ 1][2] += dp[i][parity][2];
//                         dp[i + 1][parity ^ 1][2] += 1;
//                     } else {
//                         // dp[i + 1][parity ^ 1][0] += dp[i][parity][2];
//                         dp[i + 1][parity ^ 1][0] += 1;
//                     }
//                 } else {
//                     // AC
//                     // dp[i + 1][parity ^ 1][0] += dp[i][parity][2];
//                     dp[i + 1][parity ^ 1][0] += 1;
//                 }

//                 // dp[i + 1][parity][1] += 0;
//                 // dp[i + 1][parity][2] += 0;
//             }
//             _ => unreachable!(),
//         }
//     }
// }

// dbg!(&dp);

//     let ans = *dp[n].iter().flatten().max().unwrap();
//     vis!(ans);
// }

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
