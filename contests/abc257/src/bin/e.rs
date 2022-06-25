#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        c: [usize; 9]
    }

    let s = c
        .iter()
        .copied()
        .enumerate()
        .rev()
        .min_by_key(|x| x.1)
        .unwrap();
    let m = n / s.1;

    if m == 0 {
        vis!(0);
        return;
    }

    let mut ans = vec![s.0; m];
    let mut rest = n - m * s.1;

    for i in 0..m {
        for r in (0..9).rev() {
            if r > ans[i] && c[r] - s.1 <= rest {
                rest -= c[r] - s.1;
                ans[i] = r;
                continue;
            }
        }
    }

    let ans = ans.into_iter().map(|x| x + 1).join("");
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
