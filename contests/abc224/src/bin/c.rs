//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        p: [(i64, i64); n]
    }

    let mut ans = 0usize;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i < j && j < k {
                    if !linear(p[i], p[j], p[k]) {
                        ans += 1;
                    }
                }
            }
        }
    }

    vis!(ans);
}

fn linear((x1, y1): (i64, i64), (x2, y2): (i64, i64), (x3, y3): (i64, i64)) -> bool {
    if (x1, y1) == (x2, y2) || (x2, y2) == (x3, y3) || (x3, y3) == (x1, y1) {
        true
    } else {
        let ax = x1 - x2;
        let ay = y1 - y2;
        let bx = x2 - x3;
        let by = y2 - y3;

        ax * by == bx * ay
    }
}

#[test]
fn feature() {
    dbg!(linear((0, 0), (1, 1), (2, 2)));
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
};
