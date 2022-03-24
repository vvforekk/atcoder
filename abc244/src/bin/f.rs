//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

const INF: usize = 20000000000;
// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let g = g;

    let mut dp = vec![vec![INF; 1 << 17 + 1]; n];
    dp.iter_mut().for_each(|l| l[0] = 0);

    for start in 0..n {
        let mut q = VecDeque::new();
        q.push_back((start, 1 << start, 1));

        while let Some((cur, is_odd, path)) = q.pop_front() {
            if path < dp[cur][is_odd] {
                dp[cur][is_odd] = path;
            } else {
                continue;
            }

            for &next in g[cur].iter() {
                q.push_back((next, is_odd ^ (1 << next), path + 1));
            }
        }
    }

    let ans = (0..(1 << n))
        .map(|s| dp.iter().map(|x| x[s]).min().unwrap())
        .sum::<usize>();
    vis!(ans);
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
