fn main() {
    proconiocc::input! {
        n: usize, l: usize,
        k: usize,
        mut a: [usize; n]
    }

    let mut bs = vec![a[0]];
    for i in 0..n {
        bs.push(if i != n - 1 {
            a[i + 1] - a[i]
        } else {
            l - a[i]
        })
    }

    // dbg!(bs);
}
