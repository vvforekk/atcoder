use num::Integer;

const MOD: usize = 1000000007;

fn main() {
    proconio::input! {
        n: usize, b: usize, k: usize,
        c: [usize; k]
    }

    let mut a = vec![0usize; n + 1];
    a[0] = 1;
    for i in 1..=n {
        a[i] = a[i - 1]
            * c.iter()
                .filter(|&&x| is_mul(x, i - 1, b))
                .count()
            % MOD;
    }

    println!("{}", a[n]);
}

// number n * 10 ^ e is multiple of m?
// 2 <= m <= 1000
// e <= 10e18 <- !??
fn is_mul(n: usize, e: usize, m: usize) -> bool {


    todo!()
}