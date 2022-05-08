//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        t: usize,
        q: [(usize, Chars); t]
    }

    for (n, s) in q {
        let m = (n + 1) / 2;
        let mut dp = vec![vec![ModInt998244353::zero(); 2]; n + 1];
        dp[0][1] = 1.into();
        for i in 0..m {
            let (a, b) = (dp[i][0], dp[i][1]);
            dp[i + 1][0] = b * (s[i] as u8 - b'A') + a * 26;
            dp[i + 1][1] = if s[i] as u8 <= s[n - i - 1] as u8 {
                b
            } else {
                0.into()
            };
        }
        for i in m..n {
            let (a, b) = (dp[i][0], dp[i][1]);
            dp[i + 1][0] = b + a;
            dp[i + 1][1] = b;
        }

        dbg!(&dp);
        let ans = dp[n][0] + dp[n][1];
        vis!(ans.get());
    }
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
