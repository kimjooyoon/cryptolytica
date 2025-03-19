// order_events.rs
//
// 이 파일은 exchange-domain에서 발생하는 주문 관련 이벤트를 구독하여
// trading-domain의 주문 캐시를 업데이트하는 핸들러를 정의합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use cryptolytica_shared_kernel::events::{Event, EventHandler};
use cryptolytica_shared_kernel::error::CoreError;
use cryptolytica_shared_kernel::types::{Decimal, OrderId, OrderSide, OrderStatus, OrderType, Result, SymbolPair};
use crate::model::exchange_view::order::{OrderCache, OrderView};

/// 주문 생성 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    /// 이벤트 ID
    pub id: uuid::Uuid,
    /// 이벤트 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 주문 ID
    pub order_id: OrderId,
    /// 거래소 이름
    pub exchange: String,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 주문 타입
    pub order_type: OrderType,
    /// 매수/매도
    pub side: OrderSide,
    /// 수량
    pub quantity: Decimal,
    /// 가격 (시장가 주문은 None)
    pub price: Option<Decimal>,
    /// 클라이언트 주문 ID
    pub client_order_id: Option<String>,
}

impl Event for OrderCreatedEvent {
    fn event_type(&self) -> &'static str {
        "exchange.order.created"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// 주문 상태 업데이트 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderStatusUpdatedEvent {
    /// 이벤트 ID
    pub id: uuid::Uuid,
    /// 이벤트 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 주문 ID
    pub order_id: OrderId,
    /// 거래소 이름
    pub exchange: String,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 새 주문 상태
    pub status: OrderStatus,
    /// 체결된 수량
    pub filled_quantity: Option<Decimal>,
    /// 평균 체결 가격
    pub average_fill_price: Option<Decimal>,
    /// 취소 이유 (취소된 경우)
    pub cancel_reason: Option<String>,
}

impl Event for OrderStatusUpdatedEvent {
    fn event_type(&self) -> &'static str {
        "exchange.order.status_updated"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// 주문 체결 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFilledEvent {
    /// 이벤트 ID
    pub id: uuid::Uuid,
    /// 이벤트 타임스탬프
    pub timestamp: DateTime<Utc>,
    /// 주문 ID
    pub order_id: OrderId,
    /// 거래소 이름
    pub exchange: String,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 체결 수량
    pub fill_quantity: Decimal,
    /// 체결 가격
    pub fill_price: Decimal,
    /// 누적 체결 수량
    pub total_filled: Decimal,
    /// 평균 체결 가격
    pub average_fill_price: Decimal,
    /// 수수료
    pub fee: Option<Decimal>,
    /// 수수료 자산
    pub fee_asset: Option<String>,
    /// 체결 완료 여부
    pub is_complete: bool,
}

impl Event for OrderFilledEvent {
    fn event_type(&self) -> &'static str {
        "exchange.order.filled"
    }
    
    fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}

/// 주문 생성 이벤트 핸들러
pub struct OrderCreatedEventHandler {
    order_cache: Arc<OrderCache>,
}

impl OrderCreatedEventHandler {
    /// 새로운 주문 생성 이벤트 핸들러 생성
    pub fn new(order_cache: Arc<OrderCache>) -> Self {
        Self { order_cache }
    }
}

impl EventHandler<OrderCreatedEvent> for OrderCreatedEventHandler {
    /// 주문 생성 이벤트 처리
    fn handle(&self, event: &OrderCreatedEvent) -> Result<()> {
        // 이벤트로부터 OrderView 생성
        let order_view = OrderView::new(
            event.order_id,
            event.exchange.clone(),
            event.symbol_pair.clone(),
            event.order_type,
            event.side,
            event.quantity,
            event.price,
            event.client_order_id.clone(),
        );
        
        // 주문 캐시에 추가
        self.order_cache.upsert_order(order_view);
        
        tracing::debug!(
            "주문 생성됨: {} - {} {} {} @ {:?}",
            event.order_id,
            event.exchange,
            event.symbol_pair,
            event.side,
            event.price
        );
        
        Ok(())
    }
}

/// 주문 상태 업데이트 이벤트 핸들러
pub struct OrderStatusUpdatedEventHandler {
    order_cache: Arc<OrderCache>,
}

impl OrderStatusUpdatedEventHandler {
    /// 새로운 주문 상태 업데이트 이벤트 핸들러 생성
    pub fn new(order_cache: Arc<OrderCache>) -> Self {
        Self { order_cache }
    }
}

impl EventHandler<OrderStatusUpdatedEvent> for OrderStatusUpdatedEventHandler {
    /// 주문 상태 업데이트 이벤트 처리
    fn handle(&self, event: &OrderStatusUpdatedEvent) -> Result<()> {
        // 주문 상태 업데이트
        self.order_cache.update_order_status(
            &event.order_id,
            event.status,
            event.timestamp,
        );
        
        // 체결 정보가 있으면 체결 정보도 업데이트
        if let Some(filled_quantity) = event.filled_quantity {
            self.order_cache.update_order_fill(
                &event.order_id,
                filled_quantity,
                event.average_fill_price,
                event.timestamp,
            );
        }
        
        tracing::debug!(
            "주문 상태 업데이트됨: {} - {} - {:?}",
            event.order_id,
            event.exchange,
            event.status
        );
        
        Ok(())
    }
}

/// 주문 체결 이벤트 핸들러
pub struct OrderFilledEventHandler {
    order_cache: Arc<OrderCache>,
}

impl OrderFilledEventHandler {
    /// 새로운 주문 체결 이벤트 핸들러 생성
    pub fn new(order_cache: Arc<OrderCache>) -> Self {
        Self { order_cache }
    }
}

impl EventHandler<OrderFilledEvent> for OrderFilledEventHandler {
    /// 주문 체결 이벤트 처리
    fn handle(&self, event: &OrderFilledEvent) -> Result<()> {
        // 주문 체결 정보 업데이트
        self.order_cache.update_order_fill(
            &event.order_id,
            event.total_filled,
            Some(event.average_fill_price),
            event.timestamp,
        );
        
        // 완전 체결되었으면 상태를 완료로 변경
        if event.is_complete {
            self.order_cache.update_order_status(
                &event.order_id,
                OrderStatus::Filled,
                event.timestamp,
            );
        } else {
            self.order_cache.update_order_status(
                &event.order_id,
                OrderStatus::PartiallyFilled,
                event.timestamp,
            );
        }
        
        tracing::debug!(
            "주문 체결됨: {} - {} - 수량: {}, 가격: {}, 완료: {}",
            event.order_id,
            event.exchange,
            event.fill_quantity,
            event.fill_price,
            event.is_complete
        );
        
        Ok(())
    }
} 