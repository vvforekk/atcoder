use itertools::Itertools;
use proconio::marker::Chars;

fn main() {
    proconio::input! {
        n: usize,
        s: [Chars; n]
    }

    for line in s.iter() {
        for window in line.windows(6) {
            if window.iter().filter(|&&c| c == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }

    for line in 0..n {
        for window in s.iter().map(|s| s[line]).collect_vec().windows(6) {
            if window.iter().filter(|&&c| c == '#').count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i + 5 < n && j + 5 < n {
                if (0..6)
                    .map(|k| s[i + k][j + k])
                    .filter(|&c| c == '#')
                    .count()
                    >= 4
                {
                    println!("Yes");
                    return;
                }
            }

            if i + 5 < n && j >= 5 {
                if (0..6)
                    .map(|k| s[i + k][j - k])
                    .filter(|&c| c == '#')
                    .count()
                    >= 4
                {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
