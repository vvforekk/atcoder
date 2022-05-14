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
        q: usize,
        qs: [(usize, usize); q]
    }

    let mut rng = thread_rng();
    let mut table = HashMap::new();
    let mut ac = vec![0];
    let mut used = HashSet::new();
    for &e in a.iter() {
        if used.contains(&e) {
            ac.push(*ac.last().unwrap());
        } else {
            used.insert(e);
            if !table.contains_key(&e) {
                let r: u64 = rng.gen();
                table.insert(e, r);
            }
            ac.push(ac.last().unwrap() ^ table[&e]);
        }
    }

    let mut bc = vec![0];
    let mut used = HashSet::new();
    for &e in b.iter() {
        if used.contains(&e) {
            bc.push(*bc.last().unwrap());
        } else {
            used.insert(e);
            if !table.contains_key(&e) {
                let r: u64 = rng.gen();
                table.insert(e, r);
            }
            bc.push(bc.last().unwrap() ^ table[&e]);
        }
    }

    for (x, y) in qs {
        Yn(ac[x] == bc[y]);
    }
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
    thread_rng, Rng, SeedableRng,
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
