//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        x: i64, y: i64,
        a: [i64; n]
    }

    let stmax = SparseTable::<Max<i64>>::from(a.clone());
    let stmin = SparseTable::<Min<i64>>::from(a.clone());

    let mut ans = 0;
    let s = (0..=n).collect_vec();
    for i in 0..n {
        let r = s[i + 1..n].equal_range_by_key(&(x, y), |&f| (stmax.range(i..f), stmin.range(i..f)));
        dbg!(&r);
        ans += r.end - r.start;
    }

    vis!(ans);
}

use sail::{
    prelude::*,
    sparse_table::{Max, Min, SparseTable},
};

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
use superslice::Ext;
