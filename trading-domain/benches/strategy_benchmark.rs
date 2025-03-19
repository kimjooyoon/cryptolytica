use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 간단한 더미 함수
fn dummy_strategy_calc(n: u64) -> u64 {
    // 간단한 연산 (실제 전략 계산 시뮬레이션)
    (1..n).fold(0, |a, b| a + b)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dummy strategy calc 1000", |b| {
        b.iter(|| dummy_strategy_calc(black_box(1000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 