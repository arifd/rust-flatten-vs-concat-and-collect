use criterion::{black_box, criterion_group, criterion_main, Criterion};

const QUANTITY: usize = 100;

pub fn concat(v: Vec<Vec<usize>>) -> Vec<usize> {
    v.concat()
}

pub fn iter_flatten_collect(v: Vec<Vec<usize>>) -> Vec<usize> {
    v.into_iter().flatten().collect()
}

pub fn map_nested_flatten_collect() -> Vec<usize> {
    (0..QUANTITY).map(|i| vec![i; QUANTITY]).flatten().collect()
}

pub fn map_nested_concat() -> Vec<usize> {
    let v = (0..QUANTITY).map(|i| vec![i; QUANTITY]).collect::<Vec<_>>();
    v.concat()
}

pub fn extend_flatten(v: Vec<Vec<usize>>) -> Vec<usize> {
    let mut result = Vec::with_capacity(QUANTITY * QUANTITY);
    result.extend(v.into_iter().flatten());
    result
}

pub fn result_flatten() -> Vec<usize> {
    let v: Vec<Result<usize, ()>> = vec![Ok(1); QUANTITY * QUANTITY];
    v.into_iter().flatten().collect()
}

pub fn result_collect() -> Vec<usize> {
    let v = vec![Ok(1); QUANTITY * QUANTITY];
    v.into_iter().collect::<Result<_, ()>>().unwrap()
}

pub fn option_flatten() -> Vec<usize> {
    let v = vec![Some(1); QUANTITY * QUANTITY];
    v.into_iter().flatten().collect()
}

pub fn option_collect() -> Vec<usize> {
    let v = vec![Some(1); QUANTITY * QUANTITY];
    v.into_iter().collect::<Option<_>>().unwrap()
}

fn benchmark_concat_vs_flatten_collect(c: &mut Criterion) {
    let v: Vec<Vec<usize>> = vec![vec![1; QUANTITY]; QUANTITY];

    // cmpare flatten vs concant
    c.bench_function("flatten_collect", |b| {
        b.iter(|| black_box(iter_flatten_collect(v.clone())))
    });
    c.bench_function("concat", |b| b.iter(|| black_box(concat(v.clone()))));

    // compare flatten when its part of an iterator chain building the vec vs
    // build it entirely then concat
    c.bench_function("nested_flatten_collect", |b| {
        b.iter(|| black_box(map_nested_flatten_collect()))
    });
    c.bench_function("nested_concat", |b| {
        b.iter(|| black_box(map_nested_concat()))
    });

    // compare flatten when the final vec is preallocated
    c.bench_function("extend_flatten", |b| {
        b.iter(|| black_box(extend_flatten(v.clone())))
    });

    // compare flatten vs collect
    c.bench_function("resulT_flatten", |b| b.iter(|| black_box(result_flatten())));
    c.bench_function("result_collect", |b| b.iter(|| black_box(result_collect())));
    c.bench_function("option_flatten", |b| b.iter(|| black_box(option_flatten())));
    c.bench_function("option_collect", |b| b.iter(|| black_box(option_collect())));
}

criterion_group!(benches, benchmark_concat_vs_flatten_collect);
criterion_main!(benches);
