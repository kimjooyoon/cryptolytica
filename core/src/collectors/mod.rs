//! 암호화폐 거래소 데이터 수집기 모듈
//!
//! 이 모듈은 다양한 암호화폐 거래소에서 데이터를 수집하는 인터페이스와 구현체를 제공합니다.

pub mod exchange;
pub mod binance;
pub mod coinbase;
pub mod upbit;
pub mod common;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use std::time::Duration;

use crate::error::CollectorError;
use crate::models::{Candle, OrderBook, Ticker, Trade};

/// 거래소 데이터 수집기 트레이트
#[async_trait]
pub trait ExchangeCollector: Send + Sync {
    /// 거래소 이름 반환
    fn name(&self) -> &str;
    
    /// 거래소 설명 반환
    fn description(&self) -> &str;
    
    /// 지원하는 심볼 목록 조회
    async fn get_symbols(&self) -> Result<Vec<String>, CollectorError>;
    
    /// 현재 시세 정보 조회
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker, CollectorError>;
    
    /// 호가창 정보 조회
    async fn get_order_book(&self, symbol: &str, depth: u32) -> Result<OrderBook, CollectorError>;
    
    /// 최근 거래 내역 조회
    async fn get_recent_trades(&self, symbol: &str, limit: u32) -> Result<Vec<Trade>, CollectorError>;
    
    /// 과거 캔들 데이터 조회
    async fn get_candles(
        &self,
        symbol: &str,
        interval: &str,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        limit: Option<u32>,
    ) -> Result<Vec<Candle>, CollectorError>;
    
    /// 실시간 시세 구독
    async fn subscribe_ticker(&self, symbol: &str) -> Result<(), CollectorError>;
    
    /// 실시간 호가창 구독
    async fn subscribe_order_book(&self, symbol: &str) -> Result<(), CollectorError>;
    
    /// 실시간 거래 내역 구독
    async fn subscribe_trades(&self, symbol: &str) -> Result<(), CollectorError>;
    
    /// 구독 취소
    async fn unsubscribe(&self, symbol: &str) -> Result<(), CollectorError>;
}

/// 데이터 수집기 팩토리
pub struct CollectorFactory;

impl CollectorFactory {
    /// 지정된 거래소에 대한 데이터 수집기 생성
    pub fn create(exchange: &str, api_key: Option<String>, api_secret: Option<String>) -> Result<Arc<dyn ExchangeCollector>, CollectorError> {
        match exchange.to_lowercase().as_str() {
            "binance" => {
                #[cfg(feature = "binance")]
                {
                    let collector = binance::BinanceCollector::new(api_key, api_secret)?;
                    Ok(Arc::new(collector))
                }
                #[cfg(not(feature = "binance"))]
                {
                    Err(CollectorError::UnsupportedExchange(exchange.to_string()))
                }
            },
            "coinbase" => {
                #[cfg(feature = "coinbase")]
                {
                    let collector = coinbase::CoinbaseCollector::new(api_key, api_secret)?;
                    Ok(Arc::new(collector))
                }
                #[cfg(not(feature = "coinbase"))]
                {
                    Err(CollectorError::UnsupportedExchange(exchange.to_string()))
                }
            },
            "upbit" => {
                #[cfg(feature = "upbit")]
                {
                    let collector = upbit::UpbitCollector::new(api_key, api_secret)?;
                    Ok(Arc::new(collector))
                }
                #[cfg(not(feature = "upbit"))]
                {
                    Err(CollectorError::UnsupportedExchange(exchange.to_string()))
                }
            },
            _ => Err(CollectorError::UnsupportedExchange(exchange.to_string())),
        }
    }
    
    /// 지원하는 모든 거래소 목록 반환
    pub fn supported_exchanges() -> Vec<&'static str> {
        let mut exchanges = Vec::new();
        
        #[cfg(feature = "binance")]
        exchanges.push("binance");
        
        #[cfg(feature = "coinbase")]
        exchanges.push("coinbase");
        
        #[cfg(feature = "upbit")]
        exchanges.push("upbit");
        
        exchanges
    }
}