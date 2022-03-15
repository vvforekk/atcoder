use itertools::Itertools;
use num::bigint::BigInt;
use num::Integer;
use proconio::marker::{Chars, Usize1};

fn main() {
    proconio::input! {
        s: Chars,
        q: usize,
        queries: [(usize, Usize1); q]
    }

    for (t, k) in queries {
        let (d, r) = if t < 64 { k.div_rem(&(1 << t)) } else { (0, k) };
        // dbg!(d, r);
        // assert!(r < 1 << 63);
        let c = s[d];

        let one = r.count_ones();
        let zero = t - one as usize;

        // dbg!(one, zero);

        let mut delta = (zero as i32 - one as i32) % 3;
        if delta < 0 {
            delta += 3;
        }
        // dbg!(delta);
        let ans = ((c as u8 - 'A' as u8 + delta as u8) % 3 + 'A' as u8) as char;
        println!("{}", ans);
    }
}
