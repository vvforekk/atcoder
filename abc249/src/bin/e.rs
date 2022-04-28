//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, p: i64
    }

    let mut dp = vec![vec![0i64; n + 1]; n + 1]; // t -> s
    dp[0][0] = 1;

    for i in 0..n {
        let m = if i == 0 { 26 } else { 25 };
        for j in 0..n {
            for e in (0..).take_while(|&x| 10usize.pow(x) <= n) {
                // dp[i + 1][ j + 10 ^ e .. j + 10 ^ (e + 1) ] += dp[i][j] * m;
                let mut a = Imos::new(n + 1);
                a.range_add(dp[i][j] * m % p, j + 10usize.pow(e)..min(n, j + 10usize.pow(e + 1)));
                dp[i + 1] = a.accumulation().to_vec();
            }
        }
    }

    dbg!(&dp);

    let ans = dp.iter().map(|x| x[n]).fold(0, |mut acc, x| {
        acc += x;
        acc %= p;
        acc
    });

    vis!(ans);
}

use sail::{prelude::*, accumulate::imos::Imos};

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
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
    usize::MAX,
};
use superslice::{Ext as _, Ext2 as _};
