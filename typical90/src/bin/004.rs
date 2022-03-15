use itertools::Itertools;

fn main() {
    proconio::input! {
        h: usize, w: usize,
        mat: [[usize; w]; h]
    }

    let mut horizontal = vec![0; h];
    let mut vertical = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            horizontal[i] += mat[i][j];
            vertical[j] += mat[i][j];
        }
    }

    let s = (0..h)
        .map(|i| {
            (0..w)
                .map(|j| horizontal[i] + vertical[j] - mat[i][j])
                .join(" ")
        })
        .join("\n");
    println!("{}", s);
}
