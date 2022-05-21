//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut prev = vec![None; n];
    let mut succ = vec![None; n];
    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            1 => {
                input! {
                    x: Usize1, y: Usize1
                }
                prev[y] = Some(x);
                succ[x] = Some(y);

            }
            2 => {
                input! {
                    x: Usize1, y: Usize1
                }
                prev[y] = None;
                succ[x] = None;
            }
            3 => {
                input! {
                    x: Usize1
                }
                let mut cur = x;
                while let Some(l) = prev[cur] {
                    cur = l;
                }

                let mut ans = vec![cur + 1];
                while let Some(l) = succ[cur] {
                    ans.push(l + 1);
                    cur = l;
                }

                vis!(ans.len(), ans);
            }
            _ => unreachable!(),
        }
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
    Rng, SeedableRng,
};
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
};
