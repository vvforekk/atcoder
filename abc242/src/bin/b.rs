use proconio::marker::Chars;

fn main() {
    proconio::input! {
        mut s: Chars
    }

    s.sort();

    println!("{}", s.into_iter().collect::<String>());
}
