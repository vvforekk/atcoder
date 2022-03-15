fn main() {
    proconio::input! {
        a: usize, b: usize, c: usize, x: usize,
    }

    let ans = if x <= a {
        1.
    } else if a < x && x <= b {
        (c as f64) / (b - a) as f64
    } else {
        0.
    };

    println!("{}", ans);
}
