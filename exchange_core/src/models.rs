//! 거래소 API 데이터 모델 정의
//!
//! 이 모듈은 거래소 API와 통신할 때 사용되는 데이터 구조체들을 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use cryptolytica_common_core::types::{SymbolPair, ExchangeId};

/// 주문 방향(매수/매도)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl std::fmt::Display for OrderSide {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "buy"),
            OrderSide::Sell => write!(f, "sell"),
        }
    }
}

/// 주문 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "stop_loss")]
    StopLoss,
    #[serde(rename = "stop_limit")]
    StopLimit,
    #[serde(rename = "take_profit")]
    TakeProfit,
    #[serde(rename = "take_profit_limit")]
    TakeProfitLimit,
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderType::Market => write!(f, "market"),
            OrderType::Limit => write!(f, "limit"),
            OrderType::StopLoss => write!(f, "stop_loss"),
            OrderType::StopLimit => write!(f, "stop_limit"),
            OrderType::TakeProfit => write!(f, "take_profit"),
            OrderType::TakeProfitLimit => write!(f, "take_profit_limit"),
        }
    }
}

/// 주문 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Open => write!(f, "open"),
            OrderStatus::Closed => write!(f, "closed"),
            OrderStatus::Canceled => write!(f, "canceled"),
            OrderStatus::Expired => write!(f, "expired"),
            OrderStatus::Rejected => write!(f, "rejected"),
            OrderStatus::PartiallyFilled => write!(f, "partially_filled"),
        }
    }
}

/// 오더북 항목(가격/수량)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBookEntry {
    pub price: f64,
    pub amount: f64,
}

/// 오더북(매수/매도 주문 목록)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderBook {
    pub symbol: SymbolPair,
    pub bids: Vec<OrderBookEntry>,
    pub asks: Vec<OrderBookEntry>,
    pub timestamp: DateTime<Utc>,
    pub exchange: ExchangeId,
}

/// 거래 내역
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeHistory {
    pub id: String,
    pub symbol: SymbolPair,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
    pub cost: f64,
    pub fee: Option<Fee>,
    pub timestamp: DateTime<Utc>,
}

/// 수수료 정보
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fee {
    pub cost: f64,
    pub currency: String,
    pub rate: Option<f64>,
}

/// 계정 잔고 정보
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalance {
    pub currency: String,
    pub free: f64,
    pub used: f64,
    pub total: f64,
}

impl AccountBalance {
    pub fn new(currency: impl Into<String>, free: f64, used: f64) -> Self {
        let free = free;
        let used = used;
        Self {
            currency: currency.into(),
            free,
            used,
            total: free + used,
        }
    }
}

/// 주문 정보
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub client_order_id: Option<String>,
    pub symbol: SymbolPair,
    pub side: OrderSide,
    pub type_: OrderType,
    pub status: OrderStatus,
    pub price: Option<f64>,
    pub amount: f64,
    pub filled: f64,
    pub remaining: f64,
    pub cost: f64,
    pub fee: Option<Fee>,
    pub timestamp: DateTime<Utc>,
    pub last_update: Option<DateTime<Utc>>,
    // 거래소별 추가 정보
    pub info: HashMap<String, serde_json::Value>,
}

/// 심볼 제약 정보
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolConstraints {
    pub symbol: SymbolPair,
    pub price_precision: u8,
    pub amount_precision: u8,
    pub min_amount: f64,
    pub min_cost: Option<f64>,
    pub max_amount: Option<f64>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}

/// 거래소 정보
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeInfo {
    pub id: ExchangeId,
    pub name: String,
    pub symbols: Vec<SymbolPair>,
    pub symbol_constraints: HashMap<String, SymbolConstraints>,
    pub timeframes: Vec<String>,
    pub has_websocket: bool,
    pub rate_limits: HashMap<String, u32>,
    pub features: HashMap<String, bool>,
    pub urls: HashMap<String, String>,
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_side_display() {
        assert_eq!(OrderSide::Buy.to_string(), "buy");
        assert_eq!(OrderSide::Sell.to_string(), "sell");
    }
    
    #[test]
    fn test_account_balance() {
        let balance = AccountBalance::new("BTC", 1.0, 0.5);
        assert_eq!(balance.currency, "BTC");
        assert_eq!(balance.free, 1.0);
        assert_eq!(balance.used, 0.5);
        assert_eq!(balance.total, 1.5);
    }
} 