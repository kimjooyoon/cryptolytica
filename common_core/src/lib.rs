//! CryptoLytica Common Core - 공통 타입과 유틸리티 라이브러리
//!
//! 이 라이브러리는 CryptoLytica 프로젝트의 모든 모듈에서 공통으로 사용되는
//! 핵심 타입, 유틸리티, 에러 처리 등을 제공합니다.

pub mod error;
pub mod types;
pub mod events;
pub mod utils;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 라이브러리 초기화
pub fn init() {
    println!("CryptoLytica Common Core v{} initialized", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 