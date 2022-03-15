use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    let ascii = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        .chars()
        .collect_vec();
    assert_eq!(ascii.len(), 26);
    proconio::input! {
        t: usize,
        q: [(usize, Chars); t]
    }

    for (n, s) in q {
        for i in 0..n / 2 {
            for &ch in ascii.iter() {
                if ch < s[i] {
                    
                } else if ch == s[i] {
                } else {
                    break;
                }
            }
        }
    }
}
