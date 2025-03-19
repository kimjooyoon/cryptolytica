//! 거래소 인터페이스 정의
//!
//! 이 모듈은 모든 거래소 구현이 따라야 하는 공통 인터페이스를 정의합니다.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use cryptolytica_common_core::types::{SymbolPair, Timeframe, Candle, Price, ExchangeId, AssetType};
use crate::error::Result;
use crate::models::{OrderBook, OrderSide, OrderType, OrderStatus, TradeHistory, AccountBalance, Order, ExchangeInfo};

/// 거래소 인증 정보
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeCredentials {
    /// API 키
    pub api_key: String,
    /// API 비밀 키
    pub api_secret: String,
    /// 추가 인증 정보 (일부 거래소는 추가 패스프레이즈 등이 필요)
    pub extra_params: Option<std::collections::HashMap<String, String>>,
}

/// 거래소 구성 정보
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExchangeConfig {
    /// 거래소 식별자
    pub id: ExchangeId,
    /// 거래소 이름
    pub name: String,
    /// 거래소 기본 URL
    pub base_url: String,
    /// 인증 정보 (선택적)
    pub credentials: Option<ExchangeCredentials>,
    /// 요청 제한 시간 (밀리초)
    pub timeout_ms: u64,
    /// 웹소켓 엔드포인트 URL
    pub websocket_url: Option<String>,
    /// API 요청 속도 제한 (초당 요청 수)
    pub rate_limits: Option<std::collections::HashMap<String, f64>>,
    /// 고급 설정 옵션
    pub options: Option<std::collections::HashMap<String, String>>,
}

/// 시장 데이터 획득 기능 인터페이스
#[async_trait]
pub trait MarketDataProvider: Send + Sync {
    /// 거래소 정보 조회
    async fn get_exchange_info(&self) -> Result<ExchangeInfo>;
    
    /// 지원되는 모든 심볼(거래 쌍) 목록 조회
    async fn get_symbols(&self) -> Result<Vec<SymbolPair>>;
    
    /// 실시간 시세 조회
    async fn get_ticker(&self, symbol: &SymbolPair) -> Result<Price>;
    
    /// 여러 심볼의 실시간 시세 조회
    async fn get_tickers(&self, symbols: &[SymbolPair]) -> Result<Vec<Price>>;
    
    /// 오더북 조회
    async fn get_order_book(&self, symbol: &SymbolPair, depth: Option<u32>) -> Result<OrderBook>;
    
    /// OHLCV(캔들) 데이터 조회
    async fn get_candles(
        &self, 
        symbol: &SymbolPair, 
        timeframe: Timeframe, 
        since: Option<DateTime<Utc>>, 
        limit: Option<u32>
    ) -> Result<Vec<Candle>>;
    
    /// 최근 거래 내역 조회
    async fn get_trades(
        &self, 
        symbol: &SymbolPair, 
        since: Option<DateTime<Utc>>, 
        limit: Option<u32>
    ) -> Result<Vec<TradeHistory>>;
}

/// 트레이딩 기능 인터페이스
#[async_trait]
pub trait TradingProvider: Send + Sync {
    /// 계정 잔고 조회
    async fn get_balances(&self) -> Result<Vec<AccountBalance>>;
    
    /// 주문 생성
    async fn create_order(
        &self, 
        symbol: &SymbolPair, 
        side: OrderSide, 
        order_type: OrderType, 
        amount: f64, 
        price: Option<f64>,
        params: Option<std::collections::HashMap<String, String>>
    ) -> Result<Order>;
    
    /// 주문 취소
    async fn cancel_order(&self, symbol: &SymbolPair, order_id: &str) -> Result<Order>;
    
    /// 주문 조회
    async fn get_order(&self, symbol: &SymbolPair, order_id: &str) -> Result<Order>;
    
    /// 진행 중인 주문 목록 조회
    async fn get_open_orders(&self, symbol: Option<&SymbolPair>) -> Result<Vec<Order>>;
    
    /// 완료된 주문 내역 조회
    async fn get_order_history(
        &self, 
        symbol: Option<&SymbolPair>, 
        since: Option<DateTime<Utc>>, 
        limit: Option<u32>
    ) -> Result<Vec<Order>>;
    
    /// 트레이드 내역 조회
    async fn get_my_trades(
        &self, 
        symbol: Option<&SymbolPair>, 
        since: Option<DateTime<Utc>>, 
        limit: Option<u32>
    ) -> Result<Vec<TradeHistory>>;
}

/// 완전한 거래소 API 인터페이스
#[async_trait]
pub trait Exchange: MarketDataProvider + TradingProvider {
    /// 거래소 ID 조회
    fn id(&self) -> &ExchangeId;
    
    /// 거래소 이름 조회
    fn name(&self) -> &str;
    
    /// 지원되는 자산 유형 목록 조회
    fn supported_asset_types(&self) -> Vec<AssetType>;
    
    /// 거래소 특정 기능 지원 여부 확인
    fn has_feature(&self, feature_name: &str) -> bool;
    
    /// API 속도 제한 상태 확인
    fn get_rate_limit_status(&self) -> std::collections::HashMap<String, (u32, u32)>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exchange_config() {
        let config = ExchangeConfig {
            id: ExchangeId("binance".to_string()),
            name: "Binance".to_string(),
            base_url: "https://api.binance.com".to_string(),
            credentials: Some(ExchangeCredentials {
                api_key: "test_key".to_string(),
                api_secret: "test_secret".to_string(),
                extra_params: None,
            }),
            timeout_ms: 5000,
            websocket_url: Some("wss://stream.binance.com:9443/ws".to_string()),
            rate_limits: None,
            options: None,
        };
        
        assert_eq!(config.id.0, "binance");
        assert_eq!(config.name, "Binance");
        assert!(config.credentials.is_some());
    }
} 