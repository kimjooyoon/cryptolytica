//! CryptoLytica Trading Core - 트레이딩 전략 및 실행 라이브러리
//!
//! 이 라이브러리는 암호화폐 트레이딩 전략 정의, 백테스팅, 실시간 트레이딩을
//! 위한 핵심 기능들을 제공합니다.

pub mod strategy;
pub mod backtest;
pub mod execution;
pub mod risk;
pub mod portfolio;
pub mod metrics;
pub mod error;
pub mod models;

use cryptolytica_common_core as common;
use cryptolytica_exchange_core as exchange;
use cryptolytica_market_data_core as market_data;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 트레이딩 코어 초기화
pub fn init() {
    println!("CryptoLytica Trading Core v{} initialized", VERSION);
    common::init();
    exchange::init();
    market_data::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 