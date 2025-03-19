use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use cryptolytica_shared_kernel::types::{SymbolPair, Timeframe};
use cryptolytica_shared_kernel::events::EventHandler;
use crate::model::market_view::MarketDataCache;
use crate::events::market_events::{
    PriceUpdatedEvent, PriceUpdatedEventHandler,
    CandlestickUpdatedEvent, CandlestickUpdatedEventHandler
};

#[test]
fn test_price_updated_event_handler() {
    // 설정
    let market_data_cache = Arc::new(MarketDataCache::new());
    let handler = PriceUpdatedEventHandler::new(market_data_cache.clone());
    
    let symbol_pair = SymbolPair::new("BTC", "USDT");
    let price = dec!(50000);
    let high_24h = dec!(51000);
    let low_24h = dec!(49000);
    let volume_24h = dec!(1000);
    
    let event = PriceUpdatedEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        symbol_pair: symbol_pair.clone(),
        price,
        high_24h: Some(high_24h),
        low_24h: Some(low_24h),
        volume_24h: Some(volume_24h),
    };
    
    // 실행
    let result = handler.handle(&event);
    
    // 검증
    assert!(result.is_ok());
    let price_view = market_data_cache.get_price(&symbol_pair);
    assert!(price_view.is_some());
    
    let price_view = price_view.unwrap();
    assert_eq!(price_view.price, price);
    assert_eq!(price_view.high_24h.unwrap(), high_24h);
    assert_eq!(price_view.low_24h.unwrap(), low_24h);
    assert_eq!(price_view.volume_24h.unwrap(), volume_24h);
}

#[test]
fn test_candlestick_updated_event_handler() {
    // 설정
    let market_data_cache = Arc::new(MarketDataCache::new());
    let handler = CandlestickUpdatedEventHandler::new(market_data_cache.clone());
    
    let symbol_pair = SymbolPair::new("ETH", "USDT");
    let timeframe = Timeframe::Minute1;
    let now = Utc::now();
    
    let event = CandlestickUpdatedEvent {
        id: Uuid::new_v4(),
        timestamp: now,
        symbol_pair: symbol_pair.clone(),
        timeframe,
        candle_timestamp: now,
        open: dec!(2000),
        high: dec!(2100),
        low: dec!(1950),
        close: dec!(2050),
        volume: dec!(500),
        is_complete: true,
    };
    
    // 실행
    let result = handler.handle(&event);
    
    // 검증
    assert!(result.is_ok());
    
    // 캔들스틱 캐시 검증
    let candles = market_data_cache.get_candles(&symbol_pair, timeframe);
    assert!(!candles.is_empty());
    assert_eq!(candles[0].open, dec!(2000));
    assert_eq!(candles[0].high, dec!(2100));
    assert_eq!(candles[0].low, dec!(1950));
    assert_eq!(candles[0].close, dec!(2050));
    assert_eq!(candles[0].volume, dec!(500));
    
    // 가격 캐시도 업데이트 되었는지 검증 (is_complete가 true이므로)
    let price_view = market_data_cache.get_price(&symbol_pair);
    assert!(price_view.is_some());
    assert_eq!(price_view.unwrap().price, dec!(2050)); // close 가격
}

#[test]
fn test_incomplete_candlestick_event() {
    // 설정
    let market_data_cache = Arc::new(MarketDataCache::new());
    let handler = CandlestickUpdatedEventHandler::new(market_data_cache.clone());
    
    let symbol_pair = SymbolPair::new("ETH", "USDT");
    let timeframe = Timeframe::Minute1;
    let now = Utc::now();
    
    // 먼저 초기 가격 설정
    let init_price = dec!(1900);
    market_data_cache.update_price(symbol_pair.clone(), init_price, now);
    
    // 완료되지 않은 캔들스틱 이벤트
    let event = CandlestickUpdatedEvent {
        id: Uuid::new_v4(),
        timestamp: now,
        symbol_pair: symbol_pair.clone(),
        timeframe,
        candle_timestamp: now,
        open: dec!(2000),
        high: dec!(2100),
        low: dec!(1950),
        close: dec!(2050),
        volume: dec!(500),
        is_complete: false, // 완료되지 않음
    };
    
    // 실행
    let result = handler.handle(&event);
    
    // 검증
    assert!(result.is_ok());
    
    // 캔들스틱은 업데이트되었지만, 가격은 여전히 초기값이어야 함
    let price_view = market_data_cache.get_price(&symbol_pair);
    assert!(price_view.is_some());
    assert_eq!(price_view.unwrap().price, init_price); // 초기 가격 유지
} 