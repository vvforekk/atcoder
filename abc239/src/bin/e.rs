//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

// #[fastout]
fn main() {
    input! {
        n: usize, q: usize,
        x: [usize; n],
        edges: [(Usize1, Usize1); n - 1],
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let g = g;
    let mut big20 = vec![Vec::<usize>::new(); n];

    dfs(&g, &x, 0, 0, &mut big20);

    // dbg!(big20);

    for _ in 0..q {
        input! {
            v: Usize1, k: Usize1,
        }

        let ans = big20[v][k];
        vis!(ans);
    }
}

fn dfs(g: &[Vec<usize>], a: &[usize], cur: usize, prev: usize, ans: &mut [Vec<usize>]) {
    let children = g[cur].iter().filter(|&&x| x != prev).copied().collect_vec();
    if children.is_empty() {
        ans[cur] = vec![a[cur]];
        return;
    }

    for &child in children.iter() {
        dfs(g, a, child, cur, ans);
    }

    let mut children = children
        .into_iter()
        .map(|x| ans[x].clone())
        .flatten()
        .collect_vec();
    children.push(a[cur]);
    ans[cur] = children
        .into_iter()
        .sorted_by_key(|&x| Reverse(x))
        .take(20)
        .collect_vec();
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
    cmp::{max, min, Reverse},
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
