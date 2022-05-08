//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut c = HashMap::new();
    let dc = (0..n)
        .map(|x| [a[x], b[x]])
        .flatten()
        .sorted()
        .dedup()
        .enumerate()
        .map(|(i, x)| {
            c.insert(x, i);
            x
        })
        .collect_vec();
    let m = dc.len();

    let mut ans = 0;
    let mut st = SegmentTree::<Additive<i64>>::new(m);
    for ((_, Reverse(b)), i) in (0..n)
        .map(|x| (a[x], Reverse(b[x])))
        .counts()
        .into_iter()
        .sorted()
    {
        st.update(c[&b], st.get(c[&b]) + i as i64);
        ans += st.range(c[&b]..) * i as i64;
    }

    vis!(ans);
}

use sail::{prelude::*, segment_tree::segment_tree::SegmentTree, sqrt_decomposition::Additive};

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
