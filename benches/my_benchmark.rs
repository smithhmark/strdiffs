use criterion::{black_box, criterion_group, criterion_main, Criterion};
use strdiffs::levenshtein::twovec_lev;
use strdiffs::levenshtein::vecvec_lev;

pub fn criterion_benchmark(c: &mut Criterion) {
    let left = black_box("123 n main st");
    let right = black_box("1217 north maine ave");
    c.bench_function("twovec lev abcVbca", |b| b.iter(|| twovec_lev(left, right)));
    c.bench_function("vecvec lev abcVbca", |b| b.iter(|| vecvec_lev(left, right)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
