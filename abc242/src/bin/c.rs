use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![vec![0usize; 9]; n];
    dp[0] = vec![1; 9];

    for i in 1..n {
        dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) % MOD;
        for j in 1..8 {
            dp[i][j] = ((dp[i - 1][j - 1] + dp[i - 1][j]) % MOD + dp[i - 1][j + 1]) % MOD;
        }
        dp[i][8] = (dp[i - 1][7] + dp[i - 1][8]) % MOD;
    }

    let mut ans = 0usize;
    for &i in dp[n - 1].iter() {
        ans += i;
        ans %= MOD;
    }
    println!("{}", ans);
}
