//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

const MOD: usize = 998244353;

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize, k: usize, s: Usize1, t: Usize1, x: Usize1,
        edges: [(Usize1, Usize1); m]
    }

    let mut g = vec![Vec::new(); n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }
    let g = g;

    let mut dp = vec![vec![vec![0usize; 2]; n]; k + 1]; // dp[rest_steps][current_node][is_even as usize]
    dp[k][s][true as usize] = 1;

    for rest in (1..=k).rev() {
        for cur in 0..n {
            for &is_even in &[false, true] {
                for &next in g[cur].iter() {
                    let flag = if next == x { !is_even } else { is_even };
                    dp[rest - 1][next][flag as usize] += dp[rest][cur][is_even as usize];
                    dp[rest - 1][next][flag as usize] %= MOD;
                }
            }
        }
    }

    let ans = dp[0][t][1];
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
