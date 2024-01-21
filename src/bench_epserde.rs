use epserde::prelude::*;
use criterion::{black_box, Criterion};

pub fn bench<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: Serialize + Deserialize + Clone,
    R: Fn(&T),
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/epserde", name));

    let mut buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            buffer.clear();
            unsafe {
                T::serialize(data, black_box(&mut buffer)).unwrap();
                black_box(());
            }
        })
    });

    group.bench_function("access", |b| {
        b.iter(|| unsafe {
            black_box(T::deserialize_eps(black_box(buffer.as_ref())).unwrap());
        })
    });
/* 
    group.bench_function("read (unvalidated)", |b| {
        b.iter(|| unsafe {
            read(black_box(T::deserialize_eps(buffer.as_ref()).unwrap()));
            black_box(());
        })
    });
*/
    group.bench_function("deserialize (unvalidated)", |b| {
        b.iter(|| {
            black_box(T::deserialize_eps(black_box(buffer.as_ref())).unwrap());
        })
    });

    crate::bench_size(name, "epserde", buffer.as_slice());

    group.finish();
}
