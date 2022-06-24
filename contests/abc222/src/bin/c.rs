#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize, m: usize,
        a: [Chars; n * 2]
    }

    let mut point = vec![0usize; 2 * n];

    for round in 0..m {
        let rank = (0..n * 2)
            .sorted_by_key(|&i| Reverse(point[i]))
            .collect_vec();
        for t in 0..n {
            if (a[rank[t * 2]][round] == 'G' && a[rank[t * 2 + 1]][round] == 'C')
                || (a[rank[t * 2]][round] == 'C' && a[rank[t * 2 + 1]][round] == 'P')
                || (a[rank[t * 2]][round] == 'P' && a[rank[t * 2 + 1]][round] == 'G')
            {
                point[rank[t * 2]] += 1;
            } else if (a[rank[t * 2]][round] == 'C' && a[rank[t * 2 + 1]][round] == 'G')
                || (a[rank[t * 2]][round] == 'P' && a[rank[t * 2 + 1]][round] == 'C')
                || (a[rank[t * 2]][round] == 'G' && a[rank[t * 2 + 1]][round] == 'P')
            {
                point[rank[t * 2 + 1]] += 1;
            }
        }
    }

    let ans = (0..n * 2)
        .sorted_by_key(|&i| Reverse(point[i]))
        .map(|x| x + 1)
        .collect_vec();

    vis!(ans);
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
