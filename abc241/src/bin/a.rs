fn main() {
    proconio::input! {
        aa: [usize; 10]
    }

    let mut ans = 0;

    for _ in 0..3 {
        ans = aa[ans];
    }

    println!("{}", ans);
}
