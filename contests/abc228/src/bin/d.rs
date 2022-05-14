//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        q: usize,
        qs: [(usize, usize); q]
    }

    let n = 1 << 20;
    let mut a = vec![-1; n];
    let mut b = (0..n).collect_vec();
    let mut uf = UnionFind::new(n);

    for (t, x) in qs {
        if t == 1 {
            let r = uf.root(x % n);
            let c = b[r];
            a[c] = x as i64;
            b[r] = max(b[r], b[uf.root(x % n + 1)]);
            uf.unite(b[r], b[r] + 1);
        } else {
            vis!(a[x % n])
        }
    }

    dbg!(&a[..5]);
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
