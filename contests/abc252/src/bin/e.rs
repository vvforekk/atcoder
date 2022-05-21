//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        e: [(Usize1, Usize1, usize); m]
    }

    let mut m = BTreeMap::new();
    let mut g = vec![Vec::new(); n];
    for (i, (a, b, c)) in e.into_iter().enumerate() {
        m.insert((a, b), i);
        m.insert((b, a), i);
        g[a].push((c, i, b));
        g[b].push((c, i, a));
    }

    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), 0));
    let mut ans = vec![0; n];

    while let Some((Reverse(cost), cur)) = q.pop() {
        for &(w, id, next) in g[cur]
            .iter()
            .filter(|&&(w, _, next)| cost + w < dist[next])
            .collect_vec()
        {
            dist[next] = cost + w;
            q.push((Reverse(cost + w), next));
            ans[next] = id;
        }
    }

    let ans = ans.into_iter().skip(1).map(|x| x + 1).collect_vec();
    vis!(ans);
}

use sail::{
    graph::{
        dijkstra::{dijkstra, dijkstra_with_path_hint},
        kruskal::kuruskal,
        union_find::UnionFind,
    },
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
};
