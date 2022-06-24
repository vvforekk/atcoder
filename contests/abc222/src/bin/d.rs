#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut dp = vec![ModInt998244353::one(); 3001];

    for i in 0..n {
        let mut s = vec![ModInt998244353::zero(); 3001];
        // for j in a[i]..=b[i] {
        //     s[j] = dp[j];
        // }
        s[a[i]..(b[i] + 1)].copy_from_slice(&dp[a[i]..(b[i] + 1)]);

        let mut next = vec![];
        let mut cur = ModInt998244353::zero();
        for j in s {
            cur += j;
            next.push(cur);
        }

        dp = next;
    }

    let ans = dp.last().unwrap();
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
use sail::prelude::*;
use superslice::{Ext as _, Ext2 as _};
