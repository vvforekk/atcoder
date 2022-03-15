use itertools::Itertools;

fn main() {
    proconio::input! {
        n: usize, k: usize,
        aa: [usize; n]
    }

    let m = k.next_power_of_two().trailing_zeros() as usize + 1;
    dbg!(m);
    let mut d = vec![vec![0usize; n]; m + 1];
    d[0] = (0..n).collect_vec();
    d[1] = (0..n).map(|x| x + aa[x]).collect_vec();

    for i in 2..=m {
        for j in 0..n {
            let r = d[i - 1][j] % n;
            d[i][j] = d[i - 1][j] + (d[i - 1][r] - d[0][r]);
        }
    }

    let mut ans = 0usize;
    for i in (0..m).rev() {
        if (1 << i) & k != 0 {
            ans += dbg!(d[i + 1][ans % n] - d[0][ans % n]);
        }
    }

    println!("{}", ans);
}
