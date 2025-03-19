// lib.rs
//! CryptoLytica 트레이딩 도메인 모듈
//!
//! 이 모듈은 트레이딩 전략 및 실행과 관련된 핵심 기능을 제공합니다.
//! 이벤트 기반 구독 방식을 통해 다른 도메인과 결합도가 낮은 방식으로 통신합니다.

// 트레이딩 도메인 버전
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// 모듈 정의
pub mod model;
pub mod events;
pub mod strategy;
pub mod error;

// 공개 타입
pub use events::service::TradingEventService;
pub use model::service::ModelService;

/// 트레이딩 도메인 서비스
/// 트레이딩 도메인의 주요 기능을 제공하는 진입점
pub struct TradingDomainService {
    model_service: model::service::ModelService,
    event_service: Option<events::service::TradingEventService>,
}

impl TradingDomainService {
    /// 새로운 트레이딩 도메인 서비스 생성
    pub fn new() -> Self {
        Self {
            model_service: model::service::ModelService::new(),
            event_service: None,
        }
    }
    
    /// 이벤트 버스 설정 및 구독 초기화
    pub fn with_event_bus(
        mut self,
        event_bus: std::sync::Arc<dyn cryptolytica_shared_kernel::events::EventBus>,
    ) -> cryptolytica_shared_kernel::types::Result<Self> {
        let market_data_cache = self.model_service.market_data_cache();
        let order_cache = self.model_service.order_cache();
        
        let mut event_service = events::service::TradingEventService::new(
            event_bus,
            market_data_cache,
            order_cache,
        );
        
        // 이벤트 구독 설정
        event_service.subscribe_to_events()?;
        
        self.event_service = Some(event_service);
        Ok(self)
    }
    
    /// 모델 서비스 접근
    pub fn model_service(&self) -> &model::service::ModelService {
        &self.model_service
    }
    
    /// 버전 정보 출력
    pub fn version() -> &'static str {
        VERSION
    }
}

impl Default for TradingDomainService {
    fn default() -> Self {
        Self::new()
    }
}

/// 트레이딩 도메인 모듈 초기화
pub fn init() {
    tracing::info!("CryptoLytica 트레이딩 도메인 초기화 - 버전 {}", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty(), "버전이 정의되어 있어야 함");
    }
}

// 이벤트 핸들러 테스트 모듈
#[cfg(test)]
pub mod tests {
    // 이벤트 핸들러 테스트
    pub mod market_events_test;
    pub mod order_events_test;
}

// 아직 구현되지 않은 모듈 스텁
pub mod strategy {
    //! 트레이딩 전략 관련 기능
    //! 
    //! 트레이딩 전략 구현 및 백테스팅을 위한 모듈
    
    /// 전략 특성
    pub trait Strategy: Send + Sync + 'static {
        /// 전략 ID
        fn id(&self) -> &str;
        
        /// 전략 이름
        fn name(&self) -> &str;
        
        /// 전략 설명
        fn description(&self) -> &str;
        
        /// 전략 실행
        fn execute(&self) -> cryptolytica_shared_kernel::types::Result<()>;
    }
}

pub mod error {
    //! 트레이딩 도메인 오류 정의
    
    use thiserror::Error;
    
    /// 트레이딩 도메인 오류
    #[derive(Debug, Error)]
    pub enum TradingError {
        /// 이벤트 관련 오류
        #[error("이벤트 오류: {0}")]
        Event(String),
        
        /// 전략 관련 오류
        #[error("전략 오류: {0}")]
        Strategy(String),
        
        /// 모델 관련 오류
        #[error("모델 오류: {0}")]
        Model(String),
        
        /// 실행 관련 오류
        #[error("실행 오류: {0}")]
        Execution(String),
        
        /// 기타 오류
        #[error("기타 오류: {0}")]
        Other(String),
    }
} 