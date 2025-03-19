//! 공통 타입 정의
//!
//! 이 모듈은 프로젝트 전체에서 사용되는 핵심 데이터 타입들을 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// 암호화폐 거래 쌍(Symbol Pair)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolPair {
    /// 기본 자산 (예: 'BTC', 'ETH')
    pub base: String,
    /// 견적 자산 (예: 'USDT', 'USD')
    pub quote: String,
}

impl SymbolPair {
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        Self {
            base: base.into(),
            quote: quote.into(),
        }
    }

    /// 표준 형식으로 반환 (예: "BTC/USDT")
    pub fn to_string(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }
}

/// 가격 데이터 타입
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub symbol: SymbolPair,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
}

/// OHLCV(시가, 고가, 저가, 종가, 거래량) 데이터
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    pub symbol: SymbolPair,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

/// 타임프레임 정의 (차트 기간)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    Minute1,
    Minute5,
    Minute15,
    Minute30,
    Hour1,
    Hour4,
    Hour12,
    Day1,
    Week1,
    Month1,
}

/// 거래소 ID 타입
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExchangeId(pub String);

/// 자산 유형
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    Spot,
    Futures,
    Option,
    Margin,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_pair() {
        let pair = SymbolPair::new("BTC", "USDT");
        assert_eq!(pair.base, "BTC");
        assert_eq!(pair.quote, "USDT");
        assert_eq!(pair.to_string(), "BTC/USDT");
    }
} 