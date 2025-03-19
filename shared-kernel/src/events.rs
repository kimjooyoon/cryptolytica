//! 이벤트 정의 모듈
//!
//! 이 모듈은 시스템 내 모든 모듈 간의 이벤트 기반 통신에 사용되는
//! 공통 이벤트 타입과 관련 기능을 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::any::TypeId;
use crate::error::CoreError;
use crate::types::Result;

/// 이벤트 기본 특성
pub trait Event: Send + Sync + 'static {
    /// 이벤트 타입 ID를 반환
    fn event_type(&self) -> &'static str;
    
    /// 이벤트 타임스탬프를 반환
    fn timestamp(&self) -> DateTime<Utc>;
    
    /// 이벤트 ID를 반환
    fn id(&self) -> &Uuid;
}

/// 이벤트 핸들러 특성
pub trait EventHandler<E: Event>: Send + Sync + 'static {
    /// 이벤트 처리
    fn handle(&self, event: &E) -> Result<()>;
}

/// 이벤트 버스 특성
pub trait EventBus: Send + Sync {
    /// 이벤트 발행
    fn publish<E: Event + Serialize>(&self, event: E) -> Result<()>;
    
    /// 이벤트 구독
    fn subscribe<E: Event + for<'de> Deserialize<'de>, H: EventHandler<E>>(
        &self, 
        handler: H
    ) -> Result<SubscriptionHandle>;
    
    /// 구독 취소
    fn unsubscribe(&self, handle: &SubscriptionHandle) -> Result<()>;
}

/// 이벤트 헤더 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    /// 이벤트 ID
    pub id: Uuid,
    /// 이벤트 발생 시간
    pub timestamp: DateTime<Utc>,
    /// 이벤트 발생 소스
    pub source: String,
    /// 이벤트 타입
    pub event_type: String,
    /// 관련 이벤트 ID (옵션)
    pub correlation_id: Option<Uuid>,
    /// 이벤트 버전
    pub version: String,
}

/// 이벤트 포장 구조체
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound(serialize = "T: Serialize", deserialize = "T: for<'d> Deserialize<'d>"))]
pub struct EventEnvelope<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    /// 이벤트 헤더
    pub header: EventHeader,
    /// 이벤트 데이터
    pub payload: T,
}

/// 구독 핸들
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionHandle {
    /// 구독 ID
    pub id: Uuid,
    /// 이벤트 타입 ID
    pub event_type_id: TypeId,
}

impl SubscriptionHandle {
    /// 새로운 구독 핸들 생성
    pub fn new<E: Event>(id: Uuid) -> Self {
        Self {
            id,
            event_type_id: TypeId::of::<E>(),
        }
    }
}

/// 인메모리 이벤트 버스 제공자 (구현 예시)
#[derive(Debug, Default)]
pub struct InMemoryEventBus {
    // 실제 구현은 infrastructure 모듈에서 수행
}

/// 이벤트 데이터를 직렬화
pub fn serialize_event<T: Serialize>(event: &T) -> Result<String> {
    serde_json::to_string(event).map_err(|e| 
        CoreError::Data(format!("이벤트 직렬화 실패: {}", e))
    )
}

/// 이벤트 데이터를 역직렬화
pub fn deserialize_event<T: for<'de> Deserialize<'de>>(data: &str) -> Result<T> {
    serde_json::from_str(data).map_err(|e| 
        CoreError::Data(format!("이벤트 역직렬화 실패: {}", e))
    )
}

/// 새 이벤트 헤더 생성
pub fn create_event_header(
    source: &str, 
    event_type: &str, 
    correlation_id: Option<Uuid>
) -> EventHeader {
    EventHeader {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        source: source.to_string(),
        event_type: event_type.to_string(),
        correlation_id,
        version: "1.0".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    }
    
    impl Event for TestEvent {
        fn event_type(&self) -> &'static str {
            "test.event"
        }
        
        fn timestamp(&self) -> DateTime<Utc> {
            self.timestamp
        }
        
        fn id(&self) -> &Uuid {
            &self.id
        }
    }
    
    #[test]
    fn test_event_serialization() {
        let event = TestEvent {
            id: Uuid::new_v4(),
            name: "Test Event".to_string(),
            timestamp: Utc::now(),
        };
        
        let serialized = serialize_event(&event).unwrap();
        let deserialized: TestEvent = deserialize_event(&serialized).unwrap();
        
        assert_eq!(event.id, deserialized.id);
        assert_eq!(event.name, deserialized.name);
        assert_eq!(
            event.timestamp.timestamp_millis(),
            deserialized.timestamp.timestamp_millis()
        );
    }
    
    #[test]
    fn test_create_event_header() {
        let correlation_id = Some(Uuid::new_v4());
        let header = create_event_header("test_source", "test.event", correlation_id);
        
        assert_eq!(header.source, "test_source");
        assert_eq!(header.event_type, "test.event");
        assert_eq!(header.correlation_id, correlation_id);
        assert_eq!(header.version, "1.0");
    }
} 