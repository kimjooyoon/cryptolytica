//! 거래소 도메인 서비스
//!
//! 이 모듈은 거래소 도메인의 핵심 비즈니스 로직을 담당하는 서비스들을 제공합니다.
//! DDD 관점에서 도메인 서비스는 특정 엔티티에 속하지 않는 비즈니스 로직을 캡슐화합니다.

pub mod exchange_service;
pub mod market_data_service;
pub mod trading_service;
pub mod connectivity_service;

pub use exchange_service::ExchangeService;
pub use market_data_service::MarketDataService;
pub use trading_service::TradingService;
pub use connectivity_service::ConnectivityService; 