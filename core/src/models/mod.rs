//! 암호화폐 데이터 모델
//!
//! 이 모듈은 암호화폐 데이터를 표현하는 구조체와 열거형을 정의합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 거래소 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exchange {
    /// 거래소 식별자
    pub id: String,
    /// 거래소 이름
    pub name: String,
    /// 거래소 설명
    pub description: Option<String>,
    /// 거래소 웹사이트 URL
    pub website: Option<String>,
    /// API 엔드포인트
    pub api_endpoint: String,
    /// 특성 및 기능
    pub features: HashMap<String, bool>,
}

/// 시장 티커 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// 심볼 (예: BTC-USD)
    pub symbol: String,
    /// 거래소 식별자
    pub exchange: String,
    /// 최신 시세
    pub price: f64,
    /// 24시간 변화율
    pub change_percent: f64,
    /// 24시간 거래량
    pub volume: f64,
    /// 24시간 최고가
    pub high_24h: f64,
    /// 24시간 최저가
    pub low_24h: f64,
    /// 정보 갱신 시간
    pub timestamp: DateTime<Utc>,
}

/// 주문 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrderSide {
    /// 매수 주문
    #[serde(rename = "buy")]
    Buy,
    /// 매도 주문
    #[serde(rename = "sell")]
    Sell,
}

/// 호가창 항목
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookEntry {
    /// 가격
    pub price: f64,
    /// 수량
    pub amount: f64,
    /// 주문 수
    pub count: Option<u32>,
}

/// 호가창 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    /// 심볼 (예: BTC-USD)
    pub symbol: String,
    /// 거래소 식별자
    pub exchange: String,
    /// 매수 주문(bid) 목록
    pub bids: Vec<OrderBookEntry>,
    /// 매도 주문(ask) 목록
    pub asks: Vec<OrderBookEntry>,
    /// 정보 갱신 시간
    pub timestamp: DateTime<Utc>,
}

/// 거래 내역
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    /// 거래 식별자
    pub id: String,
    /// 심볼 (예: BTC-USD)
    pub symbol: String,
    /// 거래소 식별자
    pub exchange: String,
    /// 거래 가격
    pub price: f64,
    /// 거래 수량
    pub amount: f64,
    /// 거래 방향
    pub side: OrderSide,
    /// 거래 시간
    pub timestamp: DateTime<Utc>,
}

/// 캔들 데이터 (OHLCV)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// 심볼 (예: BTC-USD)
    pub symbol: String,
    /// 거래소 식별자
    pub exchange: String,
    /// 시간 간격 (1m, 5m, 15m, 1h, 4h, 1d, 1w, 1M)
    pub interval: String,
    /// 캔들 시작 시간
    pub timestamp: DateTime<Utc>,
    /// 시가
    pub open: f64,
    /// 고가
    pub high: f64,
    /// 저가
    pub low: f64,
    /// 종가
    pub close: f64,
    /// 거래량
    pub volume: f64,
    /// 거래대금
    pub quote_volume: Option<f64>,
    /// 거래 수
    pub trade_count: Option<u32>,
}

/// 시장 요약 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSummary {
    /// 심볼 (예: BTC-USD)
    pub symbol: String,
    /// 거래소 식별자
    pub exchange: String,
    /// 현재 시세
    pub price: f64,
    /// 24시간 변화량
    pub change: f64,
    /// 24시간 변화율
    pub change_percent: f64,
    /// 24시간 최고가
    pub high_24h: f64,
    /// 24시간 최저가
    pub low_24h: f64,
    /// 24시간 거래량
    pub volume_24h: f64,
    /// 시가총액
    pub market_cap: Option<f64>,
    /// 정보 갱신 시간
    pub timestamp: DateTime<Utc>,
}