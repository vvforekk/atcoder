//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        p: [Usize1; n],
        q: [Usize1; n],
    }

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        uf.unite(p[i], q[i]);
    }

    let ans = uf
        .trees()
        .into_values()
        .map(|x| {
            let m = x.len();
            if m == 1 {
                1.into()
            } else if m == 2 {
                3.into()
            } else {
                let mut dp = vec![vec![ModInt998244353::zero(); 2]; m + 1];
                dp[0][0] = 1usize.into();
                for i in 1..=m {
                    dp[i][0] = dp[i - 1][1];
                    dp[i][1] = dp[i - 1][0] + dp[i - 1][1];
                }
                dbg!(m, &dp);
                dp[1][0] + dp[1][1] + dp[m][0] + dp[m][1]
            }
        })
        .product::<ModInt998244353>();
    vis!(ans.get());
}

use sail::{prelude::*, graph::union_find::UnionFind};

use im_rc::{ordmap, ordset, OrdMap, OrdSet};
use itertools::{iproduct, izip, Itertools as _};
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
    io::{Read as _, Write as _},
    str::FromStr,
    string::ToString,
    usize::MAX,
};
