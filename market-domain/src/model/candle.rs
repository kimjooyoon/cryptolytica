//! 캔들 모델 정의
//!
//! 이 모듈은 OHLCV(시가, 고가, 저가, 종가, 거래량) 캔들스틱 차트 데이터를 모델링합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::shared::types::{SymbolPair, ExchangeId, Timeframe};

/// OHLCV 캔들스틱 데이터를 표현하는 도메인 모델
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    /// 캔들 고유 식별자
    pub id: Uuid,
    
    /// 해당 심볼(거래 쌍)
    pub symbol: SymbolPair,
    
    /// 캔들 시작 시간
    pub timestamp: DateTime<Utc>,
    
    /// 시가 (period의 첫 거래 가격)
    pub open: f64,
    
    /// 고가 (period 내 최고 가격)
    pub high: f64,
    
    /// 저가 (period 내 최저 가격)
    pub low: f64,
    
    /// 종가 (period의 마지막 거래 가격)
    pub close: f64,
    
    /// 거래량 (period 내 거래된 총량)
    pub volume: f64,
    
    /// 데이터 소스(거래소)
    pub exchange: ExchangeId,
    
    /// 타임프레임
    pub timeframe: Timeframe,
    
    /// 거래대금 (거래량 * 평균가격)
    pub quote_volume: Option<f64>,
    
    /// 캔들 완성 여부 (false일 경우 현재 진행 중인 캔들)
    pub is_complete: bool,
}

impl Candle {
    /// 새로운 캔들 생성
    pub fn new(
        symbol: SymbolPair,
        timestamp: DateTime<Utc>,
        open: f64,
        high: f64,
        low: f64, 
        close: f64,
        volume: f64,
        exchange: ExchangeId,
        timeframe: Timeframe,
        quote_volume: Option<f64>,
        is_complete: bool,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            symbol,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
            exchange,
            timeframe,
            quote_volume,
            is_complete,
        }
    }
    
    /// 가격 변화(%) 계산
    pub fn price_change_percent(&self) -> f64 {
        if self.open == 0.0 {
            return 0.0;
        }
        
        ((self.close - self.open) / self.open) * 100.0
    }
    
    /// 캔들이 상승 캔들인지 확인
    pub fn is_bullish(&self) -> bool {
        self.close > self.open
    }
    
    /// 캔들이 하락 캔들인지 확인
    pub fn is_bearish(&self) -> bool {
        self.close < self.open
    }
    
    /// 캔들 가격 범위 계산 (고가 - 저가)
    pub fn range(&self) -> f64 {
        self.high - self.low
    }
    
    /// 캔들 몸통 크기 계산 (종가 - 시가의 절대값)
    pub fn body_size(&self) -> f64 {
        (self.close - self.open).abs()
    }
    
    /// 위 그림자 크기 계산
    pub fn upper_shadow(&self) -> f64 {
        if self.is_bullish() {
            self.high - self.close
        } else {
            self.high - self.open
        }
    }
    
    /// 아래 그림자 크기 계산
    pub fn lower_shadow(&self) -> f64 {
        if self.is_bullish() {
            self.open - self.low
        } else {
            self.close - self.low
        }
    }
    
    /// 캔들 종료 시간 계산
    pub fn end_time(&self) -> DateTime<Utc> {
        match self.timeframe {
            Timeframe::Minute1 => self.timestamp + chrono::Duration::minutes(1),
            Timeframe::Minute5 => self.timestamp + chrono::Duration::minutes(5),
            Timeframe::Minute15 => self.timestamp + chrono::Duration::minutes(15),
            Timeframe::Minute30 => self.timestamp + chrono::Duration::minutes(30),
            Timeframe::Hour1 => self.timestamp + chrono::Duration::hours(1),
            Timeframe::Hour4 => self.timestamp + chrono::Duration::hours(4),
            Timeframe::Hour12 => self.timestamp + chrono::Duration::hours(12),
            Timeframe::Day1 => self.timestamp + chrono::Duration::days(1),
            Timeframe::Week1 => self.timestamp + chrono::Duration::weeks(1),
            Timeframe::Month1 => self.timestamp + chrono::Duration::days(30), // 근사치
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    
    fn create_sample_candle() -> Candle {
        Candle::new(
            SymbolPair::new("BTC", "USDT"),
            Utc::now(),
            100.0,
            110.0,
            90.0,
            105.0,
            10.0,
            ExchangeId::new("binance"),
            Timeframe::Hour1,
            Some(1000.0),
            true,
        )
    }
    
    #[test]
    fn test_candle_properties() {
        let candle = create_sample_candle();
        
        assert_eq!(candle.price_change_percent(), 5.0);
        assert!(candle.is_bullish());
        assert!(!candle.is_bearish());
        assert_eq!(candle.range(), 20.0);
        assert_eq!(candle.body_size(), 5.0);
        assert_eq!(candle.upper_shadow(), 5.0);
        assert_eq!(candle.lower_shadow(), 10.0);
    }
    
    #[test]
    fn test_bearish_candle() {
        let mut candle = create_sample_candle();
        candle.open = 110.0;
        candle.close = 90.0;
        
        assert_eq!(candle.price_change_percent(), -18.18181818181818);
        assert!(!candle.is_bullish());
        assert!(candle.is_bearish());
        assert_eq!(candle.body_size(), 20.0);
        assert_eq!(candle.upper_shadow(), 0.0);
        assert_eq!(candle.lower_shadow(), 0.0);
    }
    
    #[test]
    fn test_end_time() {
        let now = Utc::now();
        let candle = Candle::new(
            SymbolPair::new("BTC", "USDT"),
            now,
            100.0,
            110.0,
            90.0,
            105.0,
            10.0,
            ExchangeId::new("binance"),
            Timeframe::Hour1,
            Some(1000.0),
            true,
        );
        
        let expected_end = now + chrono::Duration::hours(1);
        assert_eq!(candle.end_time().timestamp(), expected_end.timestamp());
    }
} 