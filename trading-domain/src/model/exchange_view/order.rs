// order.rs
//
// 이 파일은 exchange-domain의 주문 관련 모델을 trading-domain에서 사용하기 위한
// 뷰 모델을 정의합니다. 직접적인 의존성 대신 이벤트 기반 업데이트를 활용합니다.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use cryptolytica_shared_kernel::types::{Decimal, OrderId, OrderSide, OrderStatus, OrderType, SymbolPair};

/// 거래소 주문 뷰 - 주문 정보를 나타내는 간소화된 뷰 모델
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderView {
    /// 주문 ID
    pub order_id: OrderId,
    /// 거래소 이름
    pub exchange: String,
    /// 심볼 페어
    pub symbol_pair: SymbolPair,
    /// 주문 타입 (시장가, 지정가 등)
    pub order_type: OrderType,
    /// 매수/매도 구분
    pub side: OrderSide,
    /// 주문 수량
    pub quantity: Decimal,
    /// 주문 가격 (시장가 주문의 경우 None)
    pub price: Option<Decimal>,
    /// 체결된 수량
    pub filled_quantity: Decimal,
    /// 평균 체결 가격
    pub average_fill_price: Option<Decimal>,
    /// 주문 상태
    pub status: OrderStatus,
    /// 주문 생성 시간
    pub created_at: DateTime<Utc>,
    /// 마지막 업데이트 시간
    pub updated_at: DateTime<Utc>,
    /// 주문 만료 시간 (옵션)
    pub expires_at: Option<DateTime<Utc>>,
    /// 내부 주문 ID (알고리즘 트레이딩 추적용)
    pub client_order_id: Option<String>,
}

impl OrderView {
    /// 새로운 주문 뷰 생성
    pub fn new(
        order_id: OrderId,
        exchange: String,
        symbol_pair: SymbolPair,
        order_type: OrderType,
        side: OrderSide,
        quantity: Decimal,
        price: Option<Decimal>,
        client_order_id: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            order_id,
            exchange,
            symbol_pair,
            order_type,
            side,
            quantity,
            price,
            filled_quantity: Decimal::ZERO,
            average_fill_price: None,
            status: OrderStatus::Created,
            created_at: now,
            updated_at: now,
            expires_at: None,
            client_order_id,
        }
    }

    /// 주문 상태 업데이트
    pub fn update_status(&mut self, status: OrderStatus, timestamp: DateTime<Utc>) {
        self.status = status;
        self.updated_at = timestamp;
    }

    /// 체결 정보 업데이트
    pub fn update_fill(
        &mut self,
        filled_quantity: Decimal,
        average_fill_price: Option<Decimal>,
        timestamp: DateTime<Utc>,
    ) {
        self.filled_quantity = filled_quantity;
        self.average_fill_price = average_fill_price;
        self.updated_at = timestamp;
        
        // 완전 체결 여부 확인 및 상태 업데이트
        if filled_quantity >= self.quantity {
            self.status = OrderStatus::Filled;
        } else if filled_quantity > Decimal::ZERO && filled_quantity < self.quantity {
            self.status = OrderStatus::PartiallyFilled;
        }
    }
    
    /// 주문 취소 여부 확인
    pub fn is_canceled(&self) -> bool {
        matches!(self.status, OrderStatus::Canceled)
    }
    
    /// 주문 체결 여부 확인
    pub fn is_filled(&self) -> bool {
        matches!(self.status, OrderStatus::Filled)
    }
    
    /// 주문 활성 여부 확인
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            OrderStatus::Created
                | OrderStatus::New
                | OrderStatus::PartiallyFilled
        )
    }
}

/// 주문 캐시 - trading-domain에서 사용하는 주문 정보 캐시
#[derive(Debug)]
pub struct OrderCache {
    /// 주문 정보
    orders: Arc<RwLock<HashMap<OrderId, OrderView>>>,
    /// 클라이언트 주문 ID로 검색하기 위한 맵
    client_id_map: Arc<RwLock<HashMap<String, OrderId>>>,
}

impl OrderCache {
    /// 새로운 주문 캐시 생성
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
            client_id_map: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// 주문 추가/업데이트
    pub fn upsert_order(&self, order: OrderView) {
        let mut orders = self.orders.write().unwrap();
        
        // 클라이언트 ID가 있으면 매핑 추가
        if let Some(client_id) = &order.client_order_id {
            let mut client_map = self.client_id_map.write().unwrap();
            client_map.insert(client_id.clone(), order.order_id);
        }
        
        // 주문 캐시에 추가/업데이트
        orders.insert(order.order_id, order);
    }
    
    /// 주문 상태 업데이트
    pub fn update_order_status(
        &self,
        order_id: &OrderId,
        status: OrderStatus,
        timestamp: DateTime<Utc>,
    ) -> bool {
        let mut orders = self.orders.write().unwrap();
        
        if let Some(order) = orders.get_mut(order_id) {
            order.update_status(status, timestamp);
            true
        } else {
            false
        }
    }
    
    /// 주문 체결 정보 업데이트
    pub fn update_order_fill(
        &self,
        order_id: &OrderId,
        filled_quantity: Decimal,
        average_fill_price: Option<Decimal>,
        timestamp: DateTime<Utc>,
    ) -> bool {
        let mut orders = self.orders.write().unwrap();
        
        if let Some(order) = orders.get_mut(order_id) {
            order.update_fill(filled_quantity, average_fill_price, timestamp);
            true
        } else {
            false
        }
    }
    
    /// 주문 ID로 주문 조회
    pub fn get_order(&self, order_id: &OrderId) -> Option<OrderView> {
        let orders = self.orders.read().unwrap();
        orders.get(order_id).cloned()
    }
    
    /// 클라이언트 주문 ID로 주문 조회
    pub fn get_order_by_client_id(&self, client_id: &str) -> Option<OrderView> {
        let client_map = self.client_id_map.read().unwrap();
        
        if let Some(order_id) = client_map.get(client_id) {
            let orders = self.orders.read().unwrap();
            orders.get(order_id).cloned()
        } else {
            None
        }
    }
    
    /// 특정 심볼의 활성 주문 조회
    pub fn get_active_orders_by_symbol(&self, symbol_pair: &SymbolPair) -> Vec<OrderView> {
        let orders = self.orders.read().unwrap();
        
        orders
            .values()
            .filter(|order| order.symbol_pair == *symbol_pair && order.is_active())
            .cloned()
            .collect()
    }
    
    /// 모든 활성 주문 조회
    pub fn get_all_active_orders(&self) -> Vec<OrderView> {
        let orders = self.orders.read().unwrap();
        
        orders
            .values()
            .filter(|order| order.is_active())
            .cloned()
            .collect()
    }
}

impl Default for OrderCache {
    fn default() -> Self {
        Self::new()
    }
} 