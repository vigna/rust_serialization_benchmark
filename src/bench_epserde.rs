use criterion::{black_box, Criterion};
use epserde::prelude::*;

pub fn bench<T, R>(name: &'static str, c: &mut Criterion, data: &T, read: R)
where
    T: Serialize + Deserialize + Clone,
    R: Fn(&<T as DeserializeInner>::DeserType<'_>),
{
    const BUFFER_LEN: usize = 50_000_000;

    let mut group = c.benchmark_group(format!("{}/epserde", name));

    let mut buffer = Vec::with_capacity(BUFFER_LEN);
    group.bench_function("serialize", |b| {
        b.iter(|| {
            buffer.clear();
            black_box(T::serialize(data, &mut buffer)).unwrap();
        })
    });

    group.bench_function("deserialize (validated)", |b| {
        b.iter(|| {
            black_box(T::deserialize_eps(buffer.as_ref()).unwrap());
        })
    });

    let t = T::deserialize_eps(buffer.as_ref()).unwrap();

    group.bench_function("read (from deser)", |b| {
        b.iter(|| {
            black_box(read(&t));
        })
    });

    crate::bench_size(name, "epserde", buffer.as_slice());

    group.finish();
}
