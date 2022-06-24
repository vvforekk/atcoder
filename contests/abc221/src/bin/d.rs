#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        s: [(usize, usize); n]
    }

    let ic = IndexCompression::new(
        s.iter()
            .flat_map(|&(a, b)| [a, a + b])
            .collect_vec()
            .as_slice(),
    );

    let mut acc = vec![0i64; ic.len()];
    for (a, b) in s {
        acc[ic.compress(a).unwrap()] += 1;
        acc[ic.compress(a + b).unwrap()] -= 1;
    }

    let mut cur = 0i64;
    let mut sum = vec![];
    for i in acc {
        cur += i;
        sum.push(cur);
    }

    // dbg!(&sum);

    let mut ans = vec![0usize; n + 1];
    for i in 0..sum.len() - 1 {
        ans[sum[i] as usize] += ic.decompress(i + 1) - ic.decompress(i);
    }

    vis!(&ans[1..]);
}

use std::{
    cmp::{max, min, Reverse},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f32::consts::E,
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
use sail::{accumulate::Accumulation, prelude::*};
use superslice::{Ext as _, Ext2 as _};
