use num::Integer;

fn main() {
    proconio::input! {
        n: usize
    }

    if n.is_odd() {
        return;
    }
    f(n, 0, String::new());
}

fn f(n: usize, c: usize, s: String) {
    let k = s.len();
    if k == n {
        println!("{}", s);
    } else {
        if c == 0 {
            f(n, c + 1, format!("{}(", s));
        } else if c >= n - k {
            f(n, c - 1, format!("{})", s));
        } else {
            f(n, c + 1, format!("{}(", s));
            f(n, c - 1, format!("{})", s));
        }
    }
}
