//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        a: usize, n: usize,
    }

    let mut used = HashSet::new();
    used.insert(1);
    let mut q = VecDeque::new();
    q.push_back((1, 0));

    let mut ans = INF;
    while let Some((x, c)) = q.pop_front() {
        if x == n {
            ans.min_assign(c);
        }

        let d = dl(x);

        if x % 10 != 0 {
            let y = rl(x, d);
            if !used.contains(&y) {
                q.push_back((y, c + 1));
                used.insert(y);
            }
        }

        let z = x * a;
        if z < n * 10 && !used.contains(&z) {
            q.push_back((z, c + 1));
            used.insert(z);
        }
    }

    vis!(if ans == INF { -1 } else { ans });
}

fn dl(x: usize) -> u32 {
    // (1..).find(|&e| 10usize.pow(e) > x).unwrap()
    x.to_string().len() as u32
}

// fn rr(n: usize, d: u32) -> usize {
//     n % 10usize.pow(d - 1) * 10 + n / 10usize.pow(d - 1)
// }
fn rl(n: usize, d: u32) -> usize {
    n % 10usize * 10usize.pow(d - 1) + n / 10usize
}

use sail::{cmp::MinAssign, prelude::*};

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
