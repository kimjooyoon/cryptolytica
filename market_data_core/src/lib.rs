//! CryptoLytica Market Data Core - 시장 데이터 관리 라이브러리
//!
//! 이 라이브러리는 암호화폐 시장 데이터의 수집, 정규화, 저장 및 조회를 위한
//! 핵심 기능을 제공합니다.

pub mod collector;
pub mod storage;
pub mod normalization;
pub mod query;
pub mod error;
pub mod models;
pub mod schema;

use cryptolytica_common_core as common;
use cryptolytica_exchange_core as exchange;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 시장 데이터 코어 초기화
pub fn init() {
    println!("CryptoLytica Market Data Core v{} initialized", VERSION);
    common::init();
    exchange::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 