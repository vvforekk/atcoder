//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: (Usize1, Usize1),
        b: (Usize1, Usize1),
        s: [Chars; n]
    }

    let mut visited = vec![false; n * n];
    let mut d = vec![vec![vec![INF; 5]; n]; n];
    d[a.0][a.1][0] = 0;
    let mut q = VecDeque::new();
    q.push_back((a, 0));
    while let Some(((x, y), t)) = q.pop_front() {
        dbg!(d[x][y][t]);

        if visited[n * x + y] {
            continue;
        }
        visited[n * x + y] = true;

        if 0 < x && 0 < y && s[x - 1][y - 1] == '.' {
            if t == 1 {
                if d[x - 1][y - 1][1] > d[x][y][t] {
                    d[x - 1][y - 1][1] = d[x][y][t];
                    q.push_front(((x - 1, y - 1), 1));
                }
            } else {
                if d[x - 1][y - 1][1] > d[x][y][t] + 1 {
                    d[x - 1][y - 1][1] = d[x][y][t] + 1;
                    q.push_back(((x - 1, y - 1), 1));
                }
            }
        }
        if x < n - 1 && 0 < y && s[x + 1][y - 1] == '.' {
            if t == 2 {
                if d[x + 1][y - 1][2] > d[x][y][t] {
                    d[x + 1][y - 1][2] = d[x][y][t];
                    q.push_front(((x + 1, y - 1), 2));
                }
            } else {
                if d[x + 1][y - 1][2] > d[x][y][t] + 1 {
                    d[x + 1][y - 1][2] = d[x][y][t] + 1;
                    q.push_back(((x + 1, y - 1), 2));
                }
            }
        }
        if 0 < x && y < n - 1 && s[x - 1][y + 1] == '.' {
            if t == 3 {
                if d[x - 1][y + 1][3] > d[x][y][t] {
                    d[x - 1][y + 1][3] = d[x][y][t];
                    q.push_front(((x - 1, y + 1), 3));
                }
            } else {
                if d[x - 1][y + 1][3] > d[x][y][t] + 1 {
                    d[x - 1][y + 1][3] = d[x][y][t] + 1;
                    q.push_back(((x - 1, y + 1), 3));
                }
            }
        }
        if x < n - 1 && y < n - 1 && s[x + 1][y + 1] == '.' {
            if t == 4 {
                if d[x + 1][y + 1][4] > d[x][y][t] {
                    d[x + 1][y + 1][4] = d[x][y][t];
                    q.push_front(((x + 1, y + 1), 4));
                }
            } else {
                if d[x + 1][y + 1][4] > d[x][y][t] + 1 {
                    d[x + 1][y + 1][4] = d[x][y][t] + 1;
                    q.push_back(((x + 1, y + 1), 4));
                }
            }
        }
    }

    let m = *d[b.0][b.1].iter().min().unwrap();
    let ans = if m == INF { -1 } else { m };
    vis!(ans);
}

use sail::{
    graph::{DULGraph, DWLGraph},
    prelude::*,
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
use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::consts::PI,
    io::{Read, Write},
    str::FromStr,
    string::ToString,
    usize::MAX,
};
