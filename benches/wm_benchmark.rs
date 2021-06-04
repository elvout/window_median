use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use window_median::{BTreeWindow, VecWindow, WindowMedian};

fn insert<T: WindowMedian<u64>>(wm: &mut T, arr: &Vec<u64>) {
    for x in arr {
        wm.insert(*x);
    }
}

fn bench_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("Insert Throughput");

    let mut rng: StdRng = SeedableRng::seed_from_u64(123456789);
    let mut input: Vec<u64> = (0..2000000).collect();
    input.shuffle(&mut rng);

    group.throughput(Throughput::Elements(input.len() as u64));
    group.sample_size(10);
    for shift in 0..15 {
        let mut vwm = VecWindow::<u64>::new(1 << shift);

        group.bench_with_input(BenchmarkId::new("vec", shift), &input, |b, i| {
            b.iter(|| insert(&mut vwm, &i))
        });
    }
    for shift in 0..20 {
        let mut bwm = BTreeWindow::<u64>::new(1 << shift);
        group.bench_with_input(BenchmarkId::new("btree", shift), &input, |b, i| {
            b.iter(|| insert(&mut bwm, &i))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_throughput);
criterion_main!(benches);
