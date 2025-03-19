use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ndarray::{Array1, Array2};

// 간단한 샤프 비율 계산 함수
fn calculate_sharpe_ratio(returns: &[f64], risk_free_rate: f64) -> f64 {
    let returns_array = Array1::from_vec(returns.to_vec());
    
    // 평균 수익률 계산
    let mean_return = returns_array.mean().unwrap();
    
    // 표준 편차 계산
    let n = returns_array.len() as f64;
    let variance = returns_array
        .iter()
        .map(|&r| (r - mean_return).powi(2))
        .sum::<f64>() / (n - 1.0);
    let std_dev = variance.sqrt();
    
    // 샤프 비율 계산
    (mean_return - risk_free_rate) / std_dev
}

// 상관 행렬 계산 함수
fn calculate_correlation_matrix(data: &[Vec<f64>]) -> Array2<f64> {
    let n_assets = data.len();
    let n_periods = data[0].len();
    
    // 평균 계산
    let means: Vec<f64> = data
        .iter()
        .map(|asset_returns| asset_returns.iter().sum::<f64>() / n_periods as f64)
        .collect();
    
    // 상관 행렬 초기화
    let mut corr_matrix = Array2::zeros((n_assets, n_assets));
    
    for i in 0..n_assets {
        // 대각선은 1.0 (자기 자신과의 상관관계)
        corr_matrix[[i, i]] = 1.0;
        
        for j in i+1..n_assets {
            // 자산 i와 j의 공분산 계산
            let mut covariance = 0.0;
            for k in 0..n_periods {
                covariance += (data[i][k] - means[i]) * (data[j][k] - means[j]);
            }
            covariance /= (n_periods - 1) as f64;
            
            // 자산 i의 표준편차 계산
            let mut var_i = 0.0;
            for k in 0..n_periods {
                var_i += (data[i][k] - means[i]).powi(2);
            }
            var_i /= (n_periods - 1) as f64;
            let std_i = var_i.sqrt();
            
            // 자산 j의 표준편차 계산
            let mut var_j = 0.0;
            for k in 0..n_periods {
                var_j += (data[j][k] - means[j]).powi(2);
            }
            var_j /= (n_periods - 1) as f64;
            let std_j = var_j.sqrt();
            
            // 상관계수 계산
            let correlation = covariance / (std_i * std_j);
            
            // 행렬에 값 설정 (대칭 행렬)
            corr_matrix[[i, j]] = correlation;
            corr_matrix[[j, i]] = correlation;
        }
    }
    
    corr_matrix
}

fn criterion_benchmark(c: &mut Criterion) {
    // 샤프 비율 벤치마크
    let returns = vec![0.01, 0.02, -0.01, 0.03, -0.02, 0.01, 0.0, 0.02, 0.01, -0.01];
    let risk_free_rate = 0.001; // 0.1%
    
    c.bench_function("calculate sharpe ratio", |b| {
        b.iter(|| calculate_sharpe_ratio(black_box(&returns), black_box(risk_free_rate)))
    });
    
    // 상관 행렬 벤치마크
    let multi_asset_returns = vec![
        vec![0.01, 0.02, -0.01, 0.03, -0.02, 0.01, 0.0, 0.02, 0.01, -0.01],
        vec![0.02, 0.01, 0.0, -0.01, 0.01, 0.015, -0.01, 0.02, 0.005, 0.01],
        vec![-0.01, 0.01, 0.02, 0.01, -0.01, 0.005, 0.03, -0.02, 0.01, 0.0],
        vec![0.005, -0.01, 0.01, 0.02, 0.01, -0.01, 0.01, 0.0, 0.02, -0.02],
    ];
    
    c.bench_function("calculate correlation matrix", |b| {
        b.iter(|| calculate_correlation_matrix(black_box(&multi_asset_returns)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 