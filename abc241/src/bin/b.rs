fn main() {
    proconio::input! {
        n: usize, m: usize,
        aa: [usize; n],
        bb: [usize; m],
    }

    let mut am = std::collections::BTreeMap::new();
    let mut bm = std::collections::BTreeMap::new();

    for prod in aa {
        let e = am.entry(prod).or_insert(0);
        *e += 1;
    }
    for need in bb {
        let e = bm.entry(need).or_insert(0);
        *e += 1;
    }

    let ans = if bm.into_iter().all(|(k, v)| &v <= am.get(&k).unwrap_or(&0)) {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
