// model/mod.rs
//
// 트레이딩 도메인의 모델 정의

// 마켓 데이터 뷰 모델
pub mod market_view;

// 거래소 뷰 모델
pub mod exchange_view;

// 거래소 관련 뷰
pub mod exchange_view {
    pub mod order;
    
    pub use order::{OrderView, OrderCache};
}

// 모델 서비스 - 캐시된 뷰 모델 관리
pub mod service {
    use std::sync::Arc;
    use cryptolytica_shared_kernel::types::Result;
    use super::market_view::MarketDataCache;
    use super::exchange_view::order::OrderCache;
    
    /// 트레이딩 도메인의 모델 서비스
    /// 다른 도메인의 데이터를 캐시하고 관리
    pub struct ModelService {
        market_data_cache: Arc<MarketDataCache>,
        order_cache: Arc<OrderCache>,
    }
    
    impl ModelService {
        /// 새로운 모델 서비스 생성
        pub fn new() -> Self {
            Self {
                market_data_cache: Arc::new(MarketDataCache::new()),
                order_cache: Arc::new(OrderCache::new()),
            }
        }
        
        /// 마켓 데이터 캐시 접근
        pub fn market_data_cache(&self) -> Arc<MarketDataCache> {
            self.market_data_cache.clone()
        }
        
        /// 주문 캐시 접근
        pub fn order_cache(&self) -> Arc<OrderCache> {
            self.order_cache.clone()
        }
    }
    
    impl Default for ModelService {
        fn default() -> Self {
            Self::new()
        }
    }
} 