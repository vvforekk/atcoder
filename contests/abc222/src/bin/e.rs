#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize, m: usize, k: i64,
        a: [Usize1; m],
        e: [(Usize1, Usize1); n - 1]
    }

    let mut g = vec![Vec::new(); n];
    for &(a, b) in e.iter() {
        g[a].push(b);
        g[b].push(a);
    }

    let mut p = 0usize;
    let mut c = Counter::new();

    for (s, t) in a.into_iter().tuple_windows() {
        let mut q = VecDeque::new();
        q.push_back(s);
        let mut prev = vec![None; n];
        let mut visited = vec![false; n];
        while let Some(cur) = q.pop_front() {
            visited[cur] = true;
            if cur == t {
                break;
            }
            for &next in g[cur].iter().filter(|&&x| !visited[x]) {
                prev[next] = Some(cur);
                q.push_back(next);
            }
        }

        let mut cur = t;
        while let Some(prev) = prev[cur] {
            c[(min(cur, prev), max(cur, prev))] += 1;
            cur = prev;
            p += 1;
            if cur == s {
                break;
            }
        }
    }

    // k + p == 2r
    if (p as i64 + k).is_odd() || p as i64 + k < 0 || k > p as i64 {
        vis!(0usize);
        return;
    }

    let r = (p as i64 + k) / 2;

    let mut dp = vec![vec![ModInt998244353::zero(); p + 1]; c.len() + 1];
    dp[0][0] = 1usize.into();
    for (i, &v) in c.0.values().enumerate() {
        for j in 0..=p {
            let t = dp[i][j];
            dp[i + 1][j] += t;
            if j + v <= p {
                dp[i + 1][j + v] += t;
            }
        }
    }

    let ans = dp[c.len()][r as usize];
    vis!(ans.get());
}

use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
};

use ac_library_rs::math::{crt, inv_mod, pow_mod};
use indexmap::{indexmap, indexset, IndexMap, IndexSet};
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num::{
    bigint::{BigInt, BigUint, ToBigInt as _, ToBigUint as _},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    rational::{Ratio, Rational64},
    traits::{One, Pow, Zero},
};
use priority_queue::{DoublePriorityQueue, PriorityQueue};
use rand::{
    random,
    rngs::{SmallRng, ThreadRng},
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng, SeedableRng,
};
use roaring::{RoaringBitmap, RoaringTreemap};
use sail::{graph::UULGraph, prelude::*};
use superslice::{Ext as _, Ext2 as _};
