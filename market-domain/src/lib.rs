//! CryptoLytica 시장 데이터 도메인
//!
//! 이 크레이트는 암호화폐 시장 데이터와 관련된 도메인 모델, 서비스, 인터페이스를 제공합니다.
//! 시장 데이터의 표현, 저장, 조회에 관한 핵심 비즈니스 규칙을 담고 있습니다.

pub mod model;
pub mod repository;
pub mod service;
pub mod event;
pub mod error;

pub use cryptolytica_shared_kernel as shared;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 라이브러리 초기화
pub fn init() {
    println!("CryptoLytica Market Domain v{} initialized", VERSION);
    shared::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}