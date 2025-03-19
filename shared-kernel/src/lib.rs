//! CryptoLytica 공유 커널
//!
//! 이 크레이트는 모든 도메인 모듈에서 공통으로 사용되는 코드를 제공합니다.
//! 여기에는 기본 타입, 에러 처리, 유틸리티 함수 등이 포함됩니다.

pub mod error;
pub mod types;
pub mod utils;
pub mod events;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 라이브러리 초기화
pub fn init() {
    println!("CryptoLytica Shared Kernel v{} initialized", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 