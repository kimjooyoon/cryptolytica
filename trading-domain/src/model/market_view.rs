// market_view.rs
//
// 이 파일은 market-domain의 데이터 모델을 trading-domain에서 사용하기 위한
// 뷰 모델을 정의합니다. 직접적인 의존성 대신 이벤트 기반 업데이트를 활용합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use cryptolytica_shared_kernel::types::{Decimal, SymbolPair, Timeframe};

/// 마켓 시세 뷰 - 시장 가격 정보를 나타내는 간소화된 뷰 모델
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPriceView {
    /// 심볼 페어 (예: BTC/USDT)
    pub symbol_pair: SymbolPair,
    /// 현재 가격
    pub price: Decimal,
    /// 24시간 최고가
    pub high_24h: Decimal,
    /// 24시간 최저가
    pub low_24h: Decimal,
    /// 24시간 거래량
    pub volume_24h: Decimal,
    /// 마지막 업데이트 시간
    pub last_updated: DateTime<Utc>,
}

impl MarketPriceView {
    /// 새로운 마켓 시세 뷰 생성
    pub fn new(
        symbol_pair: SymbolPair,
        price: Decimal,
        high_24h: Decimal,
        low_24h: Decimal,
        volume_24h: Decimal,
    ) -> Self {
        Self {
            symbol_pair,
            price,
            high_24h,
            low_24h,
            volume_24h,
            last_updated: Utc::now(),
        }
    }

    /// 가격 업데이트
    pub fn update_price(&mut self, price: Decimal, timestamp: DateTime<Utc>) {
        self.price = price;
        self.last_updated = timestamp;
        
        // 최고가/최저가 업데이트
        if price > self.high_24h {
            self.high_24h = price;
        }
        if price < self.low_24h {
            self.low_24h = price;
        }
    }
    
    /// 24시간 정보 업데이트
    pub fn update_24h_data(&mut self, high: Decimal, low: Decimal, volume: Decimal) {
        self.high_24h = high;
        self.low_24h = low;
        self.volume_24h = volume;
        self.last_updated = Utc::now();
    }
}

/// 캔들스틱 뷰 - 캔들스틱 데이터를 나타내는 간소화된 뷰 모델
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlestickView {
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 타임프레임
    pub timeframe: Timeframe,
    /// 시작 시간
    pub timestamp: DateTime<Utc>,
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
}

impl CandlestickView {
    /// 새로운 캔들스틱 뷰 생성
    pub fn new(
        symbol_pair: SymbolPair,
        timeframe: Timeframe,
        timestamp: DateTime<Utc>,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
    ) -> Self {
        Self {
            symbol_pair,
            timeframe,
            timestamp,
            open,
            high,
            low,
            close,
            volume,
        }
    }
}

/// 마켓 데이터 캐시 - trading-domain에서 사용하는 마켓 데이터 캐시
#[derive(Debug)]
pub struct MarketDataCache {
    /// 현재 시세 정보
    prices: Arc<RwLock<HashMap<SymbolPair, MarketPriceView>>>,
    /// 캔들스틱 정보 - (심볼페어, 타임프레임) => 캔들스틱 배열
    candles: Arc<RwLock<HashMap<(SymbolPair, Timeframe), Vec<CandlestickView>>>>,
}

impl MarketDataCache {
    /// 새로운 마켓 데이터 캐시 생성
    pub fn new() -> Self {
        Self {
            prices: Arc::new(RwLock::new(HashMap::new())),
            candles: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 가격 업데이트
    pub fn update_price(
        &self,
        symbol_pair: SymbolPair,
        price: Decimal,
        timestamp: DateTime<Utc>,
    ) {
        let mut prices = self.prices.write().unwrap();
        
        if let Some(price_view) = prices.get_mut(&symbol_pair) {
            price_view.update_price(price, timestamp);
        } else {
            // 새로운 심볼 추가
            let price_view = MarketPriceView::new(
                symbol_pair.clone(),
                price,
                price,  // 초기 최고가는 현재가로 설정
                price,  // 초기 최저가는 현재가로 설정
                Decimal::ZERO,
            );
            prices.insert(symbol_pair, price_view);
        }
    }
    
    /// 캔들스틱 업데이트
    pub fn update_candle(&self, candle: CandlestickView) {
        let mut candles = self.candles.write().unwrap();
        let key = (candle.symbol_pair.clone(), candle.timeframe);
        
        // 해당 심볼과 타임프레임에 대한 캔들 배열 가져오기
        let candle_vec = candles.entry(key).or_insert_with(Vec::new);
        
        // 동일한 타임스탬프의 캔들이 있는지 확인
        for (i, existing) in candle_vec.iter_mut().enumerate() {
            if existing.timestamp == candle.timestamp {
                // 기존 캔들 업데이트
                *existing = candle;
                return;
            }
        }
        
        // 새 캔들 추가 (타임스탬프 순서로 정렬 유지)
        candle_vec.push(candle);
        candle_vec.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        // 캔들 개수 제한 (메모리 관리)
        const MAX_CANDLES: usize = 1000;
        if candle_vec.len() > MAX_CANDLES {
            *candle_vec = candle_vec.drain(candle_vec.len() - MAX_CANDLES..).collect();
        }
    }
    
    /// 시세 정보 가져오기
    pub fn get_price(&self, symbol_pair: &SymbolPair) -> Option<MarketPriceView> {
        let prices = self.prices.read().unwrap();
        prices.get(symbol_pair).cloned()
    }
    
    /// 캔들스틱 가져오기
    pub fn get_candles(
        &self,
        symbol_pair: &SymbolPair,
        timeframe: Timeframe,
        limit: usize,
    ) -> Vec<CandlestickView> {
        let candles = self.candles.read().unwrap();
        let key = (symbol_pair.clone(), timeframe);
        
        if let Some(candle_vec) = candles.get(&key) {
            let start_idx = if candle_vec.len() > limit {
                candle_vec.len() - limit
            } else {
                0
            };
            candle_vec[start_idx..].to_vec()
        } else {
            Vec::new()
        }
    }
}

impl Default for MarketDataCache {
    fn default() -> Self {
        Self::new()
    }
} 