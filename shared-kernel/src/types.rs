//! 공통 타입 정의
//!
//! 이 모듈은 프로젝트 전체에서 사용되는 핵심 데이터 타입들을 정의합니다.

use serde::{Deserialize, Serialize};
use std::fmt;

/// 암호화폐 거래 쌍(Symbol Pair)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SymbolPair {
    /// 기본 자산 (예: 'BTC', 'ETH')
    pub base: String,
    /// 견적 자산 (예: 'USDT', 'USD')
    pub quote: String,
}

impl SymbolPair {
    /// 새로운 거래 쌍 생성
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

impl fmt::Display for SymbolPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

/// 타임프레임 정의 (차트 기간)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    #[serde(rename = "1m")]
    Minute1,
    #[serde(rename = "5m")]
    Minute5,
    #[serde(rename = "15m")]
    Minute15,
    #[serde(rename = "30m")]
    Minute30,
    #[serde(rename = "1h")]
    Hour1,
    #[serde(rename = "4h")]
    Hour4,
    #[serde(rename = "12h")]
    Hour12,
    #[serde(rename = "1d")]
    Day1,
    #[serde(rename = "1w")]
    Week1,
    #[serde(rename = "1M")]
    Month1,
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Timeframe::Minute1 => "1m",
            Timeframe::Minute5 => "5m",
            Timeframe::Minute15 => "15m",
            Timeframe::Minute30 => "30m",
            Timeframe::Hour1 => "1h",
            Timeframe::Hour4 => "4h",
            Timeframe::Hour12 => "12h",
            Timeframe::Day1 => "1d",
            Timeframe::Week1 => "1w",
            Timeframe::Month1 => "1M",
        };
        write!(f, "{}", s)
    }
}

impl Timeframe {
    /// 타임프레임을 분 단위로 변환
    pub fn to_minutes(&self) -> u32 {
        match self {
            Timeframe::Minute1 => 1,
            Timeframe::Minute5 => 5,
            Timeframe::Minute15 => 15,
            Timeframe::Minute30 => 30,
            Timeframe::Hour1 => 60,
            Timeframe::Hour4 => 240,
            Timeframe::Hour12 => 720,
            Timeframe::Day1 => 1440,
            Timeframe::Week1 => 10080,
            Timeframe::Month1 => 43200, // 30일 기준
        }
    }
}

/// 거래소 ID 타입
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExchangeId(pub String);

impl fmt::Display for ExchangeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ExchangeId {
    /// 새로운 거래소 ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// 자산 유형
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "spot")]
    Spot,
    #[serde(rename = "futures")]
    Futures,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "margin")]
    Margin,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AssetType::Spot => "spot",
            AssetType::Futures => "futures",
            AssetType::Option => "option",
            AssetType::Margin => "margin",
        };
        write!(f, "{}", s)
    }
}

/// 주문 방향
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderSide {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        };
        write!(f, "{}", s)
    }
}

/// 결과 타입 단축형
pub type Result<T> = std::result::Result<T, crate::error::CoreError>;

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

    #[test]
    fn test_timeframe_to_minutes() {
        assert_eq!(Timeframe::Minute1.to_minutes(), 1);
        assert_eq!(Timeframe::Hour1.to_minutes(), 60);
        assert_eq!(Timeframe::Day1.to_minutes(), 1440);
    }
} 