//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

// #[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut c = a.clone();
    let mut d = b.clone();
    c.sort();
    d.sort();
    if c != d {
        println!("No");
        return;
    }

    let u = positions(&a);
    let v = positions(&b);

    let mut dd = vec![INF; n];
    for ele in u.keys() {
        for (&x, &y) in izip!(&u[ele], &v[ele]) {
            dd[x] = y as i64 - x as i64
        }
    }
    dbg!(&dd);

    let mut ans = true;
    for i in 0..n - 2 {
        while d[i] != 0 {
            let (x, y, z) = (dd[i], dd[i + 1], dd[i + 2]);
            dd[i] = z + 2;
            dd[i + 1] = y - 1;
            dd[i + 2] = x - 1;
            
        }
        // for _ in 0..2 {
        //     if dd[i] >= 1 {
        //         let (x, y, z) = (dd[i], dd[i + 1], dd[i + 2]);
        //         dd[i] = z + 2;
        //         dd[i + 1] = y - 1;
        //         dd[i + 2] = x - 1;
        //     }
        // }
    }
    dbg!(&dd);

    yn(ans);
}

const INF: i64 = 20000000000;

fn positions(v: &[usize]) -> BTreeMap<usize, Vec<usize>> {
    let mut res = BTreeMap::new();
    for (i, &e) in v.iter().enumerate() {
        let h = res.entry(e).or_insert(vec![]);
        h.push(i);
    }

    res
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
