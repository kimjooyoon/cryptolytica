//! 시장 데이터 도메인 모델
//!
//! 이 모듈은 시장 데이터와 관련된 도메인 모델(엔티티, 값 객체 등)을 정의합니다.

pub mod candle;
pub mod order_book;
pub mod ticker;
pub mod trade;
pub mod market_data;

pub use candle::Candle;
pub use order_book::{OrderBook, OrderBookEntry};
pub use ticker::Ticker;
pub use trade::Trade;
pub use market_data::{MarketData, MarketDataType};

use crate::shared::types::{SymbolPair, ExchangeId, Timeframe};