// events/mod.rs
//
// 도메인 간 이벤트 처리를 위한 모듈 정의

pub mod market_events;
pub mod exchange_events;

// 내부 모듈 가져오기
pub use market_events::{
    PriceUpdatedEvent, CandlestickUpdatedEvent,
    PriceUpdatedEventHandler, CandlestickUpdatedEventHandler,
};

// 거래소 이벤트 모듈
pub mod exchange_events {
    pub mod order_events;
    
    pub use order_events::{
        OrderCreatedEvent, OrderStatusUpdatedEvent, OrderFilledEvent,
        OrderCreatedEventHandler, OrderStatusUpdatedEventHandler, OrderFilledEventHandler,
    };
}

/// 이벤트 서비스 - 이벤트 구독 및 핸들러 관리
pub mod service {
    use std::sync::Arc;
    use cryptolytica_shared_kernel::events::{EventBus, EventHandler, SubscriptionHandle};
    use cryptolytica_shared_kernel::types::Result;
    use crate::model::market_view::MarketDataCache;
    use crate::model::exchange_view::order::OrderCache;
    use super::*;
    
    /// 트레이딩 도메인의 이벤트 서비스
    pub struct TradingEventService {
        event_bus: Arc<dyn EventBus>,
        market_data_cache: Arc<MarketDataCache>,
        order_cache: Arc<OrderCache>,
        subscriptions: Vec<SubscriptionHandle>,
    }
    
    impl TradingEventService {
        /// 새로운 이벤트 서비스 생성
        pub fn new(
            event_bus: Arc<dyn EventBus>,
            market_data_cache: Arc<MarketDataCache>,
            order_cache: Arc<OrderCache>,
        ) -> Self {
            Self {
                event_bus,
                market_data_cache,
                order_cache,
                subscriptions: Vec::new(),
            }
        }
        
        /// 외부 도메인 이벤트 구독 설정
        pub fn subscribe_to_events(&mut self) -> Result<()> {
            // 시장 가격 업데이트 이벤트 구독
            let price_handler = Arc::new(PriceUpdatedEventHandler::new(self.market_data_cache.clone()));
            let handle = self.event_bus.subscribe(price_handler)?;
            self.subscriptions.push(handle);
            
            // 캔들스틱 업데이트 이벤트 구독
            let candle_handler = Arc::new(CandlestickUpdatedEventHandler::new(self.market_data_cache.clone()));
            let handle = self.event_bus.subscribe(candle_handler)?;
            self.subscriptions.push(handle);
            
            // 주문 생성 이벤트 구독
            let order_created_handler = Arc::new(exchange_events::OrderCreatedEventHandler::new(self.order_cache.clone()));
            let handle = self.event_bus.subscribe(order_created_handler)?;
            self.subscriptions.push(handle);
            
            // 주문 상태 업데이트 이벤트 구독
            let order_status_handler = Arc::new(exchange_events::OrderStatusUpdatedEventHandler::new(self.order_cache.clone()));
            let handle = self.event_bus.subscribe(order_status_handler)?;
            self.subscriptions.push(handle);
            
            // 주문 체결 이벤트 구독
            let order_filled_handler = Arc::new(exchange_events::OrderFilledEventHandler::new(self.order_cache.clone()));
            let handle = self.event_bus.subscribe(order_filled_handler)?;
            self.subscriptions.push(handle);
            
            tracing::info!("트레이딩 도메인 이벤트 구독 설정 완료");
            Ok(())
        }
        
        /// 모든 구독 취소
        pub fn unsubscribe_all(&mut self) -> Result<()> {
            for handle in self.subscriptions.drain(..) {
                self.event_bus.unsubscribe(&handle)?;
            }
            
            tracing::info!("트레이딩 도메인 이벤트 구독 취소 완료");
            Ok(())
        }
    }
    
    impl Drop for TradingEventService {
        fn drop(&mut self) {
            if let Err(e) = self.unsubscribe_all() {
                tracing::error!("이벤트 구독 취소 중 오류 발생: {:?}", e);
            }
        }
    }
} 