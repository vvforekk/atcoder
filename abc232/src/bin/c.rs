//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        p: [(Usize1, Usize1); m],
        q: [(Usize1, Usize1); m],
    }

    let mut g = vec![vec![false; n]; n];
    for &(a, b) in p.iter() {
        g[a][b] = true;
        g[b][a] = true;
    }
    let mut a = (0..n).collect_vec();
    let f = permutohedron::Heap::new(&mut a);

    let mut h = vec![vec![false; n]; n];
    for &(a, b) in q.iter() {
        h[a][b] = true;
        h[b][a] = true;
    }
    let mut ans = false;
    'a: for o in f.into_iter() {
        for i in 0..n {
            for j in 0..n {
                if g[i][j] != h[o[i]][o[j]] {
                    continue 'a;
                }
            }
        }
        ans = true;
        break;
    }
    Yn(ans);
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
