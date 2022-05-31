# Crates for use in competitive programming

## General

### itertools (and itertools-num)

- `collect_vec()`, `unique()`, `counts()`,`positions()`
- `chain()`, `izip!()`, `iproduct!()`
- 正直パフォーマンスはあてにしない方がいい。

### proconio

- simple and elegant
- 内部実装は読んでおくべき

## Data Structures

### indexmap

- Python-like hashmap alternative
- key's ordering for `BTree`, inserted ordering for `Index`
- `remove()` is `swap_remove()`, so it breaks inserted order

### roaring

- transplanted from Java's library
- [ガチガチの論文](https://arxiv.org/pdf/1402.6407v4.pdf)を参照してて凄そう(小並感)
- compressed set of `u32`
- provides various method for numerical set, `insert_range()`, `remove_range()`, `max()`, `min()`, `select_nth()`, `rank()`
- できることはwavelet matrixっぽい感じ?

### im_rc vs. roaring

- persistent data-structures
- **`im_rs` wins** for DLs and development status
- クエリ系問題で使えるかも(https://noshi91.hatenablog.com/entry/2019/02/04/175100)

### rustc_hash

- **SLOW** than hashbrown/ahash
- never use
