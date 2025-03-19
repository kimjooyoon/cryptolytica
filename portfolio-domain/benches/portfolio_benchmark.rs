use criterion::{black_box, criterion_group, criterion_main, Criterion};

// 간단한 포트폴리오 계산 함수 (예시)
fn calculate_portfolio_value(asset_values: &[f64], weights: &[f64]) -> f64 {
    asset_values.iter()
        .zip(weights.iter())
        .map(|(value, weight)| value * weight)
        .sum()
}

// 간단한 포트폴리오 리밸런싱 함수 (예시)
fn rebalance_portfolio(current_weights: &[f64], target_weights: &[f64]) -> Vec<f64> {
    current_weights.iter()
        .zip(target_weights.iter())
        .map(|(current, target)| target - current)
        .collect()
}

fn criterion_benchmark(c: &mut Criterion) {
    // 포트폴리오 가치 계산 벤치마크
    let asset_values = vec![100.0, 200.0, 150.0, 300.0, 250.0];
    let weights = vec![0.2, 0.2, 0.2, 0.2, 0.2];
    
    c.bench_function("calculate portfolio value", |b| {
        b.iter(|| calculate_portfolio_value(
            black_box(&asset_values),
            black_box(&weights)
        ))
    });
    
    // 포트폴리오 리밸런싱 벤치마크
    let current_weights = vec![0.25, 0.25, 0.2, 0.15, 0.15];
    let target_weights = vec![0.2, 0.2, 0.2, 0.2, 0.2];
    
    c.bench_function("rebalance portfolio", |b| {
        b.iter(|| rebalance_portfolio(
            black_box(&current_weights),
            black_box(&target_weights)
        ))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 