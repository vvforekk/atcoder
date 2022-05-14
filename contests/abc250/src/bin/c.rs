//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        qs: [Usize1; q]
    }

    let mut a = (0..n).collect_vec();
    let mut b = (0..n).map(|x| (x, x)).collect::<HashMap<_, usize>>();

    for x in qs {
        if b[&x] + 1 < n {
            let li = b[&x];
            let ri = b[&x] + 1;
            let y = a[ri];
            a.swap(li, ri);
            b.insert(x, ri);
            b.insert(y, li);
        } else {
            let li = b[&x] - 1;
            let ri = b[&x];
            let y = a[li];
            a.swap(li, ri);
            b.insert(x, ri);
            b.insert(y, li);
        }
    }

    a.iter_mut().for_each(|h| *h += 1);
    vis!(a);
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
