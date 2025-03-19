//! 이벤트 정의 모듈
//!
//! 이 모듈은 시스템 내 모든 모듈 간의 이벤트 기반 통신에 사용되는
//! 공통 이벤트 타입과 관련 기능을 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::types::{SymbolPair, ExchangeId, Price, Candle, Timeframe};

/// 이벤트 기본 특성
pub trait Event: Send + Sync + 'static {
    /// 이벤트 타입 ID를 반환
    fn event_type(&self) -> &'static str;
    
    /// 이벤트 타임스탬프를 반환
    fn timestamp(&self) -> DateTime<Utc>;
}

/// 이벤트 헤더 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    /// 이벤트 ID (UUID)
    pub id: String,
    /// 이벤트 발생 시간
    pub timestamp: DateTime<Utc>,
    /// 이벤트 발생 소스
    pub source: String,
    /// 이벤트 타입
    pub event_type: String,
}

/// 범용 이벤트 컨테이너
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    /// 이벤트 헤더
    pub header: EventHeader,
    /// 이벤트 데이터
    pub payload: T,
}

/// 시장 데이터 이벤트 타입
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketDataEvent {
    /// 실시간 가격 업데이트
    PriceUpdate {
        exchange: ExchangeId,
        price: Price,
    },
    
    /// 캔들 업데이트
    CandleUpdate {
        exchange: ExchangeId,
        candle: Candle,
        timeframe: Timeframe,
    },
    
    /// 시장 깊이 업데이트 (오더북)
    OrderBookUpdate {
        exchange: ExchangeId,
        symbol: SymbolPair,
        bids: Vec<(f64, f64)>, // 가격, 수량
        asks: Vec<(f64, f64)>, // 가격, 수량
        timestamp: DateTime<Utc>,
    },
}

/// 시스템 이벤트 타입
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    /// 서비스 시작
    ServiceStarted {
        service_name: String,
        config_version: String,
        timestamp: DateTime<Utc>,
    },
    
    /// 서비스 중지
    ServiceStopped {
        service_name: String,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    
    /// 구성 변경
    ConfigChanged {
        service_name: String,
        old_version: String,
        new_version: String,
        timestamp: DateTime<Utc>,
    },
}

/// 이벤트 버스 관리자 인터페이스
pub trait EventBus: Send + Sync {
    /// 이벤트 발행
    fn publish<T>(&self, event: EventEnvelope<T>) -> crate::error::Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;
    
    /// 특정 이벤트 타입 구독
    fn subscribe<T, F>(&self, handler: F) -> crate::error::Result<SubscriptionHandle>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        F: Fn(EventEnvelope<T>) + Send + Sync + 'static;
    
    /// 구독 취소
    fn unsubscribe(&self, handle: SubscriptionHandle) -> crate::error::Result<()>;
}

/// 구독 핸들
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionHandle(pub String);

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    // 테스트용 더미 이벤트 버스 구현
    struct MockEventBus;
    
    impl EventBus for MockEventBus {
        fn publish<T>(&self, _event: EventEnvelope<T>) -> crate::error::Result<()>
        where
            T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
        {
            Ok(())
        }
        
        fn subscribe<T, F>(&self, _handler: F) -> crate::error::Result<SubscriptionHandle>
        where
            T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
            F: Fn(EventEnvelope<T>) + Send + Sync + 'static,
        {
            Ok(SubscriptionHandle("test-handle".to_string()))
        }
        
        fn unsubscribe(&self, _handle: SubscriptionHandle) -> crate::error::Result<()> {
            Ok(())
        }
    }
    
    #[test]
    fn test_event_bus_mock() {
        let bus = Arc::new(MockEventBus);
        let header = EventHeader {
            id: "test-id".to_string(),
            timestamp: Utc::now(),
            source: "test-source".to_string(),
            event_type: "test-type".to_string(),
        };
        
        let event = EventEnvelope {
            header,
            payload: "test-payload",
        };
        
        let result = bus.publish(event);
        assert!(result.is_ok());
    }
} 