//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

const INF: usize = 30000000000;

// #[fastout]
fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, usize); m]
    }

    let mut dist = vec![vec![INF; n]; n];
    let mut prev = vec![vec![None; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
        prev[i][i] = Some(i);
    }
    for &(a, b, c) in edges.iter() {
        dist[a][b] = c;
        dist[b][a] = c;
        prev[a][b] = Some(a);
        prev[b][a] = Some(b);
    }

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if dist[i][j] + dist[j][k] < dist[i][k] {
                    dist[i][k] = dist[i][j] + dist[j][k];
                    dist[k][i] = dist[i][j] + dist[j][k];
                    prev[i][k] = prev[j][k];
                    prev[k][i] = prev[j][k];
                }
            }
        }
    }

    let mut count = BTreeMap::new();
    for start in 0..n {
        for goal in 0..n {
            if start == goal {
                continue;
            }
            let mut cur = goal;
            let mut path = vec![];
            while cur != start {
                path.push(cur);
                cur = prev[start][cur].unwrap();
            }
            path.push(start);
            path.reverse();

            for (a, b) in path.windows(2).map(|x| (x[0], x[1])) {
                *count.entry((a, b)).or_insert(0) += 1;
                *count.entry((b, a)).or_insert(0) += 1;
            }
            // dbg!(start, goal, &path);
        }
    }

    // dbg!(&count);

    let ans = edges
        .iter()
        .filter(|&&(a, b, _)| *count.get(&(a, b)).unwrap_or(&0) <= 1)
        .count();

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
    vec,
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
