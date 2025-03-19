//! 시장 데이터 도메인 모델
//!
//! 이 모듈은 시장 데이터와 관련된 도메인 모델(엔티티, 값 객체 등)을 정의합니다.

pub mod candle;
// 아직 구현되지 않은 모듈은 주석 처리
// pub mod order_book;
// pub mod ticker;
// pub mod trade;
// pub mod market_data;

pub use candle::Candle;
// 아직 구현되지 않은 모듈의 타입 참조도 주석 처리
// pub use order_book::{OrderBook, OrderBookEntry};
// pub use ticker::Ticker;
// pub use trade::Trade;
// pub use market_data::{MarketData, MarketDataType};

// 나중에 필요할 때 다시 주석 해제
// use crate::shared::types::{SymbolPair, ExchangeId, Timeframe};