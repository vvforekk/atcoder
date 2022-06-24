//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, p: [(i64, i64); n]
    }

    let mut cur = (1, 0);
    let mut ans = 0usize;
    for (x, y) in p
        .into_iter()
        .sorted_by(|(x1, y1), (x2, y2)| (y1 * (x2 - 1)).cmp(&(y2 * (x1 - 1))))
    {
        if (y - 1) * cur.0 >= cur.1 * x {
            cur = (x - 1, y);
            ans += 1;
        }

        // dbg!(x, y, cur);
    }

    vis!(ans);
}

use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::{
        consts::{FRAC_2_PI, PI},
        EPSILON,
    },
    io::{Read, Write},
    str::FromStr,
    string::ToString,
};

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
use sail::prelude::*;
