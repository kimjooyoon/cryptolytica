// market_events.rs
//
// 이 파일은 market-domain에서 발생하는 이벤트를 구독하여
// trading-domain의 캐시된 모델을 업데이트하는 핸들러를 정의합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use cryptolytica_shared_kernel::events::{Event, EventHandler};
use cryptolytica_shared_kernel::error::CoreError;
use cryptolytica_shared_kernel::types::{Decimal, Result, SymbolPair, Timeframe};
use crate::model::market_view::{CandlestickView, MarketDataCache};

/// 가격 업데이트 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdatedEvent {
    /// 이벤트 ID
    pub id: uuid::Uuid,
    /// 이벤트 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 업데이트된 가격
    pub price: Decimal,
    /// 24시간 최고가
    pub high_24h: Option<Decimal>,
    /// 24시간 최저가
    pub low_24h: Option<Decimal>,
    /// 24시간 거래량
    pub volume_24h: Option<Decimal>,
}

impl Event for PriceUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "market.price.updated"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// 캔들스틱 업데이트 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlestickUpdatedEvent {
    /// 이벤트 ID
    pub id: uuid::Uuid,
    /// 이벤트 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 타임프레임
    pub timeframe: Timeframe,
    /// 캔들 시작 시간
    pub candle_timestamp: DateTime<Utc>,
    /// 시가
    pub open: Decimal,
    /// 고가
    pub high: Decimal,
    /// 저가
    pub low: Decimal,
    /// 종가
    pub close: Decimal,
    /// 거래량
    pub volume: Decimal,
    /// 캔들 완성 여부
    pub is_complete: bool,
}

impl Event for CandlestickUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "market.candlestick.updated"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// 가격 업데이트 이벤트 핸들러
pub struct PriceUpdatedEventHandler {
    market_data_cache: Arc<MarketDataCache>,
}

impl PriceUpdatedEventHandler {
    /// 새로운 가격 업데이트 이벤트 핸들러 생성
    pub fn new(market_data_cache: Arc<MarketDataCache>) -> Self {
        Self { market_data_cache }
    }
}

impl EventHandler<PriceUpdatedEvent> for PriceUpdatedEventHandler {
    /// 가격 업데이트 이벤트 처리
    fn handle(&self, event: &PriceUpdatedEvent) -> Result<()> {
        // 시장 데이터 캐시에 가격 업데이트
        self.market_data_cache.update_price(
            event.symbol_pair.clone(),
            event.price,
            event.timestamp,
        );
        
        // 24시간 데이터도 있으면 업데이트
        if let (Some(high), Some(low), Some(volume)) = (event.high_24h, event.low_24h, event.volume_24h) {
            if let Some(price_view) = self.market_data_cache.get_price(&event.symbol_pair) {
                // 기존 뷰를 가져와서 메모리에서 업데이트
                let mut updated_view = price_view;
                updated_view.update_24h_data(high, low, volume);
                
                // 시장 데이터 캐시 업데이트
                self.market_data_cache.update_price(
                    event.symbol_pair.clone(),
                    event.price,
                    event.timestamp,
                );
            }
        }
        
        tracing::debug!(
            "가격 업데이트됨: {} - {} @ {}",
            event.symbol_pair,
            event.price,
            event.timestamp
        );
        
        Ok(())
    }
}

/// 캔들스틱 업데이트 이벤트 핸들러
pub struct CandlestickUpdatedEventHandler {
    market_data_cache: Arc<MarketDataCache>,
}

impl CandlestickUpdatedEventHandler {
    /// 새로운 캔들스틱 업데이트 이벤트 핸들러 생성
    pub fn new(market_data_cache: Arc<MarketDataCache>) -> Self {
        Self { market_data_cache }
    }
}

impl EventHandler<CandlestickUpdatedEvent> for CandlestickUpdatedEventHandler {
    /// 캔들스틱 업데이트 이벤트 처리
    fn handle(&self, event: &CandlestickUpdatedEvent) -> Result<()> {
        // 캔들스틱 뷰 생성
        let candle = CandlestickView::new(
            event.symbol_pair.clone(),
            event.timeframe,
            event.candle_timestamp,
            event.open,
            event.high,
            event.low,
            event.close,
            event.volume,
        );
        
        // 시장 데이터 캐시에 캔들스틱 업데이트
        self.market_data_cache.update_candle(candle);
        
        // 캔들이 완성되었고, 종가가 현재 가격을 반영하는 경우 가격도 업데이트
        if event.is_complete {
            self.market_data_cache.update_price(
                event.symbol_pair.clone(),
                event.close,
                event.timestamp,
            );
            
            tracing::debug!(
                "캔들스틱 완성됨: {} - {} @ {}",
                event.symbol_pair,
                event.timeframe,
                event.candle_timestamp
            );
        } else {
            tracing::debug!(
                "캔들스틱 업데이트됨: {} - {} @ {}",
                event.symbol_pair,
                event.timeframe,
                event.candle_timestamp
            );
        }
        
        Ok(())
    }
} 