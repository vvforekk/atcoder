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
            let m: ModInt998244353 = x.len().into();
            if m.get().is_even() {
                2usize.into()
            } else {
                m
            }
        })
        .sum::<ModInt998244353>();
}

use sail::{graph::union_find::UnionFind, prelude::*};

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
