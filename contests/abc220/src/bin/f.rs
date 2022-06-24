#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        e: [(Usize1, Usize1); n - 1]
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in e {
        g[a].push(b);
        g[b].push(a);
    }

    let mut ans = vec![0usize; n];

    let mut q = VecDeque::new();
    q.push_back(0);
    let mut visited = vec![false; n];
    while let Some(cur) = q.pop_front() {
        visited[cur] = true;
        let children = g[cur].iter().filter(|&&x| !visited[x]).collect_vec();
    }
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
use sail::prelude::*;
use superslice::{Ext as _, Ext2 as _};
