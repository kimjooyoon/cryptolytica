//! CryptoLytica Exchange Core - 암호화폐 거래소 연동 라이브러리
//!
//! 이 라이브러리는 암호화폐 거래소 API 연동을 위한 기능들을 제공합니다.
//! 여러 거래소와의 인터페이스를 통일된 방식으로 제공하고, 장애 처리와 안정성을
//! 보장하는 기능을 포함합니다.

pub mod client;
pub mod exchange;
pub mod models;
pub mod websocket;
pub mod api;
pub mod error;

use cryptolytica_common_core as common;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 거래소 코어 초기화
pub fn init() {
    println!("CryptoLytica Exchange Core v{} initialized", VERSION);
    common::init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 