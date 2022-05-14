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

    let mut rest = vec![0usize; n];
    let mut g = vec![Vec::new(); n];
    for (u, v) in e {
        g[u].push(v);
        g[v].push(u);
        rest[u] += 1;
        rest[v] += 1;
    }
    let g = g;

    let mut visited = vec![false; n];
    let mut q = VecDeque::new();
    let mut ans = ModInt998244353::one();

    for start in 0..n {
        if !visited[start] {
            q.push_back(start);

            while let Some(cur) = q.pop_front() {
                ans *= rest[cur];
                visited[cur] = true;
                for &next in g[cur].iter().filter(|&&x| !visited[x]).collect_vec() {
                    rest[cur] -= 1;
                    rest[next] -= 1;
                    q.push_back(next);
                }
            }
        }
    }

    vis!(ans.get());
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
