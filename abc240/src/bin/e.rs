//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    }

    let mut tree = vec![Vec::new(); n];
    for (a, b) in edges {
        tree[a].push(b);
        tree[b].push(a);
    }
    let tree = tree;

    let mut ans = vec![(0, 0); n];
    let mut leaves = 1;

    dfs(&tree, 0, 0, &mut leaves, &mut ans);
    for (a, b) in ans {
        vis!(a, b);
    }
}

fn dfs(
    graph: &[Vec<usize>],
    cur: usize,
    prev: usize,
    leaves: &mut usize,
    ans: &mut [(usize, usize)],
) -> (usize, usize) {
    let neighbors = graph[cur].iter().filter(|&&x| x != prev).collect_vec();
    if neighbors.is_empty() {
        let res = (*leaves, *leaves);
        ans[cur] = res;
        *leaves += 1;
        // dbg!(cur);
        res
    } else {
        let l = *leaves;
        for &next in neighbors.into_iter() {
            dfs(graph, next, cur, leaves, ans);
        }
        let r = *leaves - 1;
        ans[cur] = (l, r);
        (l, r)
    }
}

use itertools::{iproduct, izip, Itertools};
use itertools_num::ItertoolsNum;
use maplit::{btreemap, btreeset, hashmap, hashset};
use num::{pow, BigInt, Bounded, Complex, Integer, One, Zero};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use rand::{random, rngs::SmallRng, Rng};
use std::{
    char::from_digit,
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    convert::{From, Into},
    str::FromStr,
    string::ToString,
};

use tools::{yn, Visualize, YN};
pub mod tools {
    use itertools::Itertools;
    pub fn yn(flag: bool) {
        println!("{}", if flag { "Yes" } else { "No" });
    }

    #[allow(non_snake_case)]
    pub fn YN(flag: bool) {
        println!("{}", if flag { "YES" } else { "NO" });
    }

    pub trait Visualize {
        fn visualize(&self, split: &str);
        fn continuous(&self) {
            self.visualize("");
        }
        fn spaces(&self) {
            self.visualize(" ");
        }
        fn lines(&self) {
            self.visualize("\n");
        }
    }

    macro_rules! impl_vis_for_sized {
        ($($t:ty),+) => {
            $(
                impl Visualize for $t {
                    fn visualize(&self, _split: &str) {
                        print!("{}", self);
                    }
                }
            )+
        };
    }

    impl_vis_for_sized! {
        usize, u8, u16, u32, u64, u128,
        isize, i8, i16, i32, i64, i128,
        String, &str, char
    }

    impl<T: std::fmt::Display> Visualize for [T] {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    #[macro_export]
    macro_rules! vis {
        // end
        () => {
            println!();
        };

        // last element + trailing pattern
        ($last:expr ;) => {
            $last.lines();
            vis!()
        };
        ($last:expr =>) => {
            $last.continuous();
            vis!();
        };
        ($last:expr $(,)?) => {
            $last.spaces();
            vis!();
        };

        // get first element and pass rest
        ($first:expr; $($rest:tt)*) => {
            $first.lines();
            println!();
            vis!($($rest)*);
        };
        ($first:expr => $($rest:tt)*) => {
            $first.continuous();
            vis!($($rest)*);
        };
        ($first:expr, $($rest:tt)*) => {
            $first.spaces();
            print!(" ");
            vis!($($rest)*);
        };
    }
}
