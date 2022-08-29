use criterion::{black_box, criterion_group, criterion_main, Criterion};
use two_buffers::{
    avx256, dummy_input, four_at_a_time, regular_loop, straightforward_sum, sum_using_iterator,
};

fn criterion_benchmark(c: &mut Criterion) {
    let mut buffer_a = dummy_input();
    buffer_a.reverse();
    let buffer_b = dummy_input();
    c.bench_function("straightforward_sum", |b| {
        b.iter(|| {
            black_box(straightforward_sum(
                black_box(&buffer_a),
                black_box(&buffer_b),
            ))
        })
    });
    c.bench_function("regular_loop", |b| {
        b.iter(|| black_box(regular_loop(black_box(&buffer_a), black_box(&buffer_b))))
    });
    c.bench_function("four_at_a_time", |b| {
        b.iter(|| black_box(four_at_a_time(black_box(&buffer_a), black_box(&buffer_b))))
    });
    #[cfg(target_arch = "x86_64")]
    unsafe {
        c.bench_function("avx256", |b| {
            b.iter(|| black_box(avx256(black_box(&buffer_a), black_box(&buffer_b))))
        });
    }
    c.bench_function("sum_using_iterator", |b| {
        b.iter(|| {
            black_box(sum_using_iterator(
                black_box(&buffer_a),
                black_box(&buffer_b),
            ))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
