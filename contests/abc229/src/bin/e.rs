//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        e: [(Usize1, Usize1); m]
    }

    let mut v = vec![Vec::new(); n];
    for &(a, b) in e.iter() {
        v[min(a, b)].push((a, b));
    }

    let mut uf = UnionFind::new(n);
    let mut t = n;
    let mut ans = vec![];
    for i in (0..n).rev() {
        ans.push(t - i - 1);
        for &(a, b) in v[i].iter() {
            if uf.is_joint(a, b) {
            } else {
                uf.unite(a, b);
                t -= 1;
            }
        }
    }

    for i in (0..n).rev() {
        vis!(ans[i]);
    }
}

use sail::{graph::union_find::UnionFind, prelude::*};

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
