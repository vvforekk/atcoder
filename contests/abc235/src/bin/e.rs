//! This solution is created by @awpsyrhy
//! Source of my library is at https://github.com/SaiYS/sail
#![allow(unused_imports)]
#![warn(clippy::dbg_macro)]

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize, q: usize,
        mut e: [(Usize1, Usize1, u64); m],
        qs: [(Usize1, Usize1, u64); q]
    }

    e.sort_by_key(|x| Reverse(x.2));
    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();

    while mst_edges.len() != n - 1 {
        let (a, b, c) = e.pop().unwrap();
        if !uf.is_joint(a, b) {
            uf.unite(a, b);
            mst_edges.push((a, b, c));
        }
    }

    let root = 0;
    let tree = UWLGraph::from_edges(n, &mst_edges);
    let mut parent = vec![root; n];
    let mut depth = vec![0usize; n];
    let mut dist = vec![0u64; n];
    let mut visited = vec![false; n];
    let mut stack = vec![(root, root)];
    let mut f = vec![0; n];
    while let Some((cur, prev)) = stack.pop() {
        visited[cur] = true;
        parent[cur] = prev;
        for (next, w) in tree
            .adjacencies(cur)
            .into_iter()
            .filter(|&(x, _)| !visited[x])
        {
            f[next] = w;
            depth[next] = depth[cur] + 1;
            dist[next] = dist[cur] + w;
            stack.push((next, cur));
        }
    }

    let mut ancestor = vec![parent];
    let mut max_e = vec![f];
    for k in 1.. {
        if ancestor.last().unwrap().iter().all(|&x| x == root) {
            break;
        }

        let next = (0..n)
            .map(|x| ancestor[k - 1][ancestor[k - 1][x]])
            .collect_vec();
        ancestor.push(next);

        let next = (0..n)
            .map(|x| max(max_e[k - 1][x], max_e[k - 1][ancestor[k - 1][x]]))
            .collect_vec();
        max_e.push(next);
    }

    // dbg!(&ancestor);
    // dbg!(&max_e);

    for (mut a, mut b, c) in qs {
        let mut da = depth[a];
        let mut db = depth[b];
        if da > db {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut da, &mut db);
        }
        let aa = a;
        let bb = b;

        let mut d = db - da;
        let mut i = 0;
        while d != 0 {
            if d & 1 != 0 {
                b = ancestor[i][b];
            }
            d >>= 1;
            i += 1;
        }

        let l = if a == b {
            a
        } else {
            for k in (0..ancestor.len()).rev() {
                if ancestor[k][a] != ancestor[k][b] {
                    a = ancestor[k][a];
                    b = ancestor[k][b];
                }
            }
            ancestor[0][a]
        };

        let mut a = aa;
        let mut b = bb;
        let da = depth[a] - depth[l];
        let db = depth[b] - depth[l];

        let mut ans = 0u64;
        for i in (0..)
            .take_while(|x| 1 << x <= da.next_power_of_two())
            .filter(|x| (1 << x) & da != 0)
        {
            ans.max_assign(max_e[i][a]);
            a = ancestor[i][a];
        }
        for i in (0..)
            .take_while(|x| 1 << x <= db.next_power_of_two())
            .filter(|x| (1 << x) & db != 0)
        {
            ans.max_assign(max_e[i][b]);
            b = ancestor[i][b];
        }

        Yn(ans > c);
    }
}

use sail::{
    graph::{
        lowest_common_ancestor::LowestCommonAncestor, union_find::UnionFind, UWLGraph,
        WeightedListGraph,
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
    usize::MAX,
};
