use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        q: usize,
    }

    let mut aa = BTreeMap::new();

    for _t in 0..q {
        input! {
            tp: usize
        }

        match tp {
            1 => {
                input! {
                    x: usize
                }
                let m = aa.entry(x).or_insert(0);
                *m += 1;
            }
            2 => {
                input! {
                    x: usize,
                    k: Usize1,
                }

                let mut ans = -1;
                let mut sum = 0usize;
                for (element, value) in aa.range(..=x).rev() {
                    sum += value;
                    if k < sum {
                        ans = *element as i64;
                        break;
                    }
                }

                println!("{}", ans);
            }
            3 => {
                input! {
                    x: usize,
                    k: Usize1,
                }

                let mut ans = -1;
                let mut sum = 0usize;
                for (element, value) in aa.range(x..) {
                    sum += value;
                    if k < sum {
                        ans = *element as i64;
                        break;
                    }
                }

                println!("{}", ans);
            }
            _ => {}
        }
    }
}
