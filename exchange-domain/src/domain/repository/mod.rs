//! 거래소 도메인 리포지토리 인터페이스
//!
//! 이 모듈은 거래소 도메인에서 사용하는 리포지토리 인터페이스들을 정의합니다.
//! DDD 관점에서 리포지토리는 도메인 객체와 영속성 계층 사이의 추상화를 제공합니다.

use async_trait::async_trait;
use uuid::Uuid;
use cryptolytica_shared_kernel::error::Result as SharedResult;

use crate::domain::model::{
    Exchange,
    ExchangeId,
    Order,
    Trade,
    Market,
};

/// 거래소 리포지토리 인터페이스
#[async_trait]
pub trait ExchangeRepository: Send + Sync {
    /// 거래소 ID로 조회
    async fn find_by_id(&self, id: Uuid) -> SharedResult<Option<Exchange>>;
    
    /// 거래소 코드로 조회
    async fn find_by_exchange_id(&self, exchange_id: &ExchangeId) -> SharedResult<Option<Exchange>>;
    
    /// 모든 거래소 조회
    async fn find_all(&self) -> SharedResult<Vec<Exchange>>;
    
    /// 활성화된 거래소만 조회
    async fn find_active(&self) -> SharedResult<Vec<Exchange>>;
    
    /// 거래소 저장
    async fn save(&self, exchange: &Exchange) -> SharedResult<Exchange>;
    
    /// 거래소 삭제
    async fn delete(&self, id: Uuid) -> SharedResult<()>;
}

/// 주문 리포지토리 인터페이스
#[async_trait]
pub trait OrderRepository: Send + Sync {
    /// 주문 ID로 조회
    async fn find_by_id(&self, id: Uuid) -> SharedResult<Option<Order>>;
    
    /// 거래소 주문 ID로 조회
    async fn find_by_exchange_order_id(&self, exchange_id: &ExchangeId, exchange_order_id: &str) -> SharedResult<Option<Order>>;
    
    /// 거래소별 주문 조회
    async fn find_by_exchange(&self, exchange_id: &ExchangeId, limit: Option<usize>) -> SharedResult<Vec<Order>>;
    
    /// 주문 저장
    async fn save(&self, order: &Order) -> SharedResult<Order>;
    
    /// 주문 벌크 저장
    async fn save_batch(&self, orders: &[Order]) -> SharedResult<Vec<Order>>;
}

/// 거래(체결) 리포지토리 인터페이스
#[async_trait]
pub trait TradeRepository: Send + Sync {
    /// 거래 ID로 조회
    async fn find_by_id(&self, id: Uuid) -> SharedResult<Option<Trade>>;
    
    /// 주문 ID로 조회
    async fn find_by_order_id(&self, order_id: Uuid) -> SharedResult<Vec<Trade>>;
    
    /// 거래소별 거래 조회
    async fn find_by_exchange(&self, exchange_id: &ExchangeId, limit: Option<usize>) -> SharedResult<Vec<Trade>>;
    
    /// 거래 저장
    async fn save(&self, trade: &Trade) -> SharedResult<Trade>;
    
    /// 거래 벌크 저장
    async fn save_batch(&self, trades: &[Trade]) -> SharedResult<Vec<Trade>>;
}

/// 마켓(거래쌍) 리포지토리 인터페이스
#[async_trait]
pub trait MarketRepository: Send + Sync {
    /// 마켓 ID로 조회
    async fn find_by_id(&self, id: Uuid) -> SharedResult<Option<Market>>;
    
    /// 거래소 및 심볼로 조회
    async fn find_by_exchange_and_symbol(&self, exchange_id: &ExchangeId, symbol: &str) -> SharedResult<Option<Market>>;
    
    /// 거래소별 마켓 조회
    async fn find_by_exchange(&self, exchange_id: &ExchangeId) -> SharedResult<Vec<Market>>;
    
    /// 모든 마켓 조회
    async fn find_all(&self) -> SharedResult<Vec<Market>>;
    
    /// 마켓 저장
    async fn save(&self, market: &Market) -> SharedResult<Market>;
    
    /// 마켓 벌크 저장
    async fn save_batch(&self, markets: &[Market]) -> SharedResult<Vec<Market>>;
} 