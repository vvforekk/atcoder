#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: Digits
    }

    let mut ans = 0usize;

    let m = n.len();
    for b in 0..1 << m {
        let mut p = vec![];
        let mut q = vec![];
        for i in 0..m {
            if (b >> i) & 1 != 0 {
                p.push(n[i]);
            } else {
                q.push(n[i]);
            }
        }

        p.sort_by_key(|&x| Reverse(x));
        q.sort_by_key(|&x| Reverse(x));
        if p.first() == None || p.first() == Some(&0) || q.first() == None || q.first() == Some(&0) {
            continue;
        }
        let p = p.into_iter().join("").parse::<usize>().unwrap();
        let q = q.into_iter().join("").parse::<usize>().unwrap();

        // dbg!(p, q);

        ans = max(ans, p * q);
    }

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
use maplit::hashset;
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
