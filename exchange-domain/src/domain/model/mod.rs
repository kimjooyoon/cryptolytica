//! 거래소 도메인 모델
//!
//! 이 모듈은 거래소 도메인의 핵심 엔티티와 값 객체를 정의합니다.

pub mod exchange;
pub mod order;
pub mod orderbook;
pub mod trade;
pub mod account;
pub mod market;
pub mod credential;

// 공통 모델 재노출
pub use exchange::Exchange;
pub use exchange::ExchangeId;
pub use exchange::ExchangeType;
pub use exchange::ExchangeStatus;
pub use order::Order;
pub use order::OrderSide;
pub use order::OrderStatus;
pub use order::OrderType;
pub use orderbook::OrderBook;
pub use orderbook::OrderBookEntry;
pub use trade::Trade;
pub use account::AccountBalance;
pub use market::Market;
pub use credential::Credential; 