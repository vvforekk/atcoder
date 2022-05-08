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

    let mut ans = true;
    let mut c = vec![0usize; n];
    for &(a, b) in &e {
        c[a] += 1;
        c[b] += 1;
    }
    if c.iter().any(|&x| x > 2) {
        ans = false;
    }
    let g = UULGraph::from_edges(n, &e);
    let mut visited = vec![false; n];
    'a: for start in 0..n {
        if !visited[start] {
            visited[start] = true;
            let mut s = vec![(start, start)];
            while let Some((prev, cur)) = s.pop() {
                for next in g.adjacencies(cur).into_iter().filter(|&x| x != prev) {
                    if visited[next] {
                        ans = false;
                        break 'a;
                    } else {
                        visited[next] = true;
                        s.push((cur, next));
                    }
                }
            }
        }
    }

    Yn(ans);
}

use sail::{graph::UULGraph, prelude::*};

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
