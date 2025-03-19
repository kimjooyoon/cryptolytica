//! CryptoLytica 인프라스트럭처 모듈
//!
//! 이 모듈은 도메인 인터페이스의 구현체를 제공합니다.
//! 데이터베이스, 메시징, HTTP 클라이언트 등 외부 시스템과의 연동을 담당합니다.

pub mod events;
pub mod repositories;
pub mod services;
pub mod adapters;

// 공개 타입
pub use events::InMemoryEventBus;

/// 인프라스트럭처 모듈 버전
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 인프라스트럭처 모듈 초기화
pub fn init() {
    tracing::info!("CryptoLytica 인프라스트럭처 초기화 - 버전 {}", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty(), "버전이 정의되어 있어야 함");
    }
}

// 아직 구현되지 않은 모듈 스텁
pub mod repositories {
    //! 저장소 구현체 모듈
}

pub mod services {
    //! 서비스 구현체 모듈
}

pub mod adapters {
    //! 외부 시스템 어댑터 모듈
} 