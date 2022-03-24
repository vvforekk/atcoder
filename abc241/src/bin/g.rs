//! This solution is created by SaiYS, @awpsyrhy(Twitter)
#![allow(unused_imports)]

// #[fastout]
fn main() {
    input! {}
    todo!("You can solve it!")
}

use im_rc::{ordmap, ordset, OrdMap, OrdSet};
use itertools::{iproduct, izip, Itertools};
use itertools_num::ItertoolsNum;
use maplit::{btreemap, btreeset, convert_args, hashmap, hashset};
use num::{
    bigint::{BigInt, BigUint, ToBigInt, ToBigUint},
    complex::Complex64,
    integer::{binomial, gcd, gcd_lcm, lcm, multinomial, Integer},
    traits::{abs, abs_sub, Bounded, One, Pow, Saturating, Zero},
};
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::{auto::AutoSource, line::LineSource, once::OnceSource},
};
use rand::{random, rngs::SmallRng, Rng};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
    convert::{From, Into},
    f64::consts::PI,
    str::FromStr,
    string::ToString,
    usize::MAX,
};

pub type HashSet<T> = rustc_hash::FxHashSet<T>;
pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

pub const MOD_1000000007: usize = 1000000007;
pub const MOD_998244353: usize = 998244353;
pub const INF: usize = 2000000000;
pub const FNI: i64 = -2000000000;
pub const ALPHABET_LARGE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABET_SMALL: &str = "abcdefghijklmnopqrstuvwxyz";
pub const ADJ4: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const ADJ8: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

use tools::{yn, Visualize, YN};
pub mod tools {
    use itertools::{Iterate, Itertools};
    use std::{
        collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
        fmt::Display,
    };

    pub type HashSet<T> = rustc_hash::FxHashSet<T>;
    pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;

    pub fn yn(flag: bool) {
        println!("{}", if flag { "Yes" } else { "No" });
    }

    #[allow(non_snake_case)]
    pub fn YN(flag: bool) {
        println!("{}", if flag { "YES" } else { "NO" });
    }

    #[macro_export]
    macro_rules! vis {
        () => {
            println!();
        };

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
        f32, f64,
        String, &str, char
    }

    impl<T: Display, U: Display> Visualize for (T, U) {
        fn visualize(&self, _split: &str) {
            print!("{} {}", self.0, self.1);
        }
    }

    impl<T: Display, U: Display, V: Display> Visualize for (T, U, V) {
        fn visualize(&self, _split: &str) {
            print!("{} {} {}", self.0, self.1, self.2);
        }
    }

    impl<T: Display, U: Display, V: Display, W: Display> Visualize for (T, U, V, W) {
        fn visualize(&self, _split: &str) {
            print!("{} {} {} {}", self.0, self.1, self.2, self.3);
        }
    }

    impl<T: Display, U: Display, V: Display, W: Display, X: Display> Visualize for (T, U, V, W, X) {
        fn visualize(&self, _split: &str) {
            print!("{} {} {} {} {}", self.0, self.1, self.2, self.3, self.4);
        }
    }

    impl<T: Display> Visualize for [T] {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<T: Display> Visualize for &[T] {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<T: Display> Visualize for VecDeque<T> {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<T: Display> Visualize for BinaryHeap<T> {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<T: Display> Visualize for HashSet<T> {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<K: Display, V: Display> Visualize for HashMap<K, V> {
        fn visualize(&self, split: &str) {
            print!(
                "{}",
                self.iter().map(|(k, v)| format!("{} {}", k, v)).join(split)
            );
        }
    }

    impl<T: Display> Visualize for BTreeSet<T> {
        fn visualize(&self, split: &str) {
            print!("{}", self.iter().join(split));
        }
    }

    impl<K: Display, V: Display> Visualize for BTreeMap<K, V> {
        fn visualize(&self, split: &str) {
            print!(
                "{}",
                self.iter().map(|(k, v)| format!("{} {}", k, v)).join(split)
            );
        }
    }
}
