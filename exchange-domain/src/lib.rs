//! Exchange Domain - 암호화폐 거래소 도메인
//!
//! 이 도메인은 암호화폐 거래소 연동 및 통신을 담당합니다.
//! 여러 거래소와의 인터페이스를 표준화하고, API 통신, 웹소켓 연결,
//! 주문 관리 등의 기능을 제공합니다.

pub mod domain;
pub mod application;
pub mod interface;
pub mod infrastructure;
pub mod config;

// 공유 커널 의존성
use cryptolytica_shared_kernel as shared_kernel;

/// 도메인 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 도메인 ID (바운디드 컨텍스트 식별자)
pub const DOMAIN_ID: &str = "exchange";

/// 도메인 초기화
pub async fn init() -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("CryptoLytica Exchange Domain v{} initializing", VERSION);
    
    // 공유 커널 초기화
    shared_kernel::init();
    
    // 설정 로드
    config::load_config()?;
    
    tracing::info!("Exchange Domain initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
} 