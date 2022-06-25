#![warn(clippy::dbg_macro)]
#![allow(unused_imports, clippy::needless_range_loop)]

fn main() {
    input! {
        n: usize,
        s: Digits,
        w: [usize; n]
    }

    let mut cc = 0usize;
    let mut ca = s.iter().filter(|&&x| x == 1).count();
    let mut ans = ca;
    let it = izip!(w, s).sorted().collect_vec();
    for i in 0..it.len() {
        if it[i].1 == 0 {
            cc += 1;
        } else if it[i].1 == 1 {
            ca -= 1;
        }

        if i == it.len() - 1 || it[i].0 != it[i + 1].0 {
            ans = max(ans, cc + ca);
        }
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
