//! 이벤트 시스템 모듈
//!
//! 이 모듈은 도메인 간 통신에 사용되는 이벤트 시스템을 정의합니다.
//! DDD 컨텍스트에서 도메인 이벤트는 도메인 내부의 상태 변화를 알리는 데 사용됩니다.
//! 이를 통해 느슨한 결합이 가능하며, 여러 바운디드 컨텍스트 간 정보 교환이 가능합니다.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::any::Any;
use std::fmt::Debug;
use std::sync::Arc;

/// 기본 이벤트 특성
pub trait Event: Send + Sync + Debug + 'static {
    /// 이벤트 타입 ID를 반환
    fn event_type(&self) -> &'static str;
    
    /// 이벤트 타임스탬프를 반환
    fn timestamp(&self) -> DateTime<Utc>;
    
    /// 이벤트 ID를 반환
    fn id(&self) -> Uuid;
    
    /// 이벤트가 발생한 도메인 (바운디드 컨텍스트)
    fn domain(&self) -> &'static str;
    
    /// 이벤트를 Any로 변환 (다운캐스팅 목적)
    fn as_any(&self) -> &dyn Any;
}

/// 이벤트 헤더 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventHeader {
    /// 이벤트 ID (UUID)
    pub id: Uuid,
    /// 이벤트 발생 시간
    pub timestamp: DateTime<Utc>,
    /// 이벤트 발생 도메인
    pub domain: String,
    /// 이벤트 타입
    pub event_type: String,
    /// 추가 메타데이터
    pub metadata: serde_json::Value,
}

impl EventHeader {
    /// 새 이벤트 헤더 생성
    pub fn new(domain: impl Into<String>, event_type: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            domain: domain.into(),
            event_type: event_type.into(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
    
    /// 메타데이터 추가
    pub fn with_metadata(mut self, key: &str, value: impl Serialize) -> Result<Self, serde_json::Error> {
        let value = serde_json::to_value(value)?;
        
        if let serde_json::Value::Object(ref mut map) = self.metadata {
            map.insert(key.to_string(), value);
        }
        
        Ok(self)
    }
}

/// 이벤트 봉투 (Envelope)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// 이벤트 헤더
    pub header: EventHeader,
    /// 이벤트 데이터
    pub payload: T,
}

impl<T> EventEnvelope<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    /// 새 이벤트 봉투 생성
    pub fn new(domain: impl Into<String>, event_type: impl Into<String>, payload: T) -> Self {
        Self {
            header: EventHeader::new(domain, event_type),
            payload,
        }
    }
}

/// 비동기 이벤트 게시자 (Publisher) 인터페이스
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// 이벤트 발행
    async fn publish<T>(&self, event: EventEnvelope<T>) -> crate::error::Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;
}

/// 비동기 이벤트 구독자 (Subscriber) 인터페이스
#[async_trait]
pub trait EventSubscriber: Send + Sync {
    /// 이벤트 타입 정보 반환
    fn event_type(&self) -> &'static str;
    
    /// 이벤트 처리
    async fn handle(&self, event_json: &str) -> crate::error::Result<()>;
}

/// 이벤트 구독 핸들
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionHandle(pub Uuid);

impl SubscriptionHandle {
    /// 새 구독 핸들 생성
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SubscriptionHandle {
    fn default() -> Self {
        Self::new()
    }
}

/// 비동기 이벤트 버스 인터페이스
#[async_trait]
pub trait EventBus: Send + Sync {
    /// 이벤트 발행
    async fn publish<T>(&self, event: EventEnvelope<T>) -> crate::error::Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;
    
    /// 이벤트 구독
    async fn subscribe<S>(&self, subscriber: S) -> crate::error::Result<SubscriptionHandle>
    where
        S: EventSubscriber + 'static;
    
    /// 구독 취소
    async fn unsubscribe(&self, handle: SubscriptionHandle) -> crate::error::Result<()>;
}

/// 인메모리 이벤트 버스 구현 (테스트 및 단일 프로세스 환경용)
#[derive(Default)]
pub struct InMemoryEventBus {
    // 구현은 생략 (실제로는 내부 필드와 구현 로직 필요)
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish<T>(&self, _event: EventEnvelope<T>) -> crate::error::Result<()>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        // 간소화된 구현 (실제 구현은 구독자들에게 이벤트 전달)
        Ok(())
    }
    
    async fn subscribe<S>(&self, _subscriber: S) -> crate::error::Result<SubscriptionHandle>
    where
        S: EventSubscriber + 'static,
    {
        // 간소화된 구현 (실제 구현은 구독자를 내부 맵에 저장)
        Ok(SubscriptionHandle::new())
    }
    
    async fn unsubscribe(&self, _handle: SubscriptionHandle) -> crate::error::Result<()> {
        // 간소화된 구현 (실제 구현은 구독자를 내부 맵에서 제거)
        Ok(())
    }
}

/// 이벤트 버스 팩토리
pub struct EventBusFactory;

impl EventBusFactory {
    /// 환경설정에 따라 적절한 이벤트 버스 구현체 생성 
    pub fn create() -> Arc<dyn EventBus> {
        // 실제 구현에서는 환경 설정에 따라 다른 이벤트 버스 구현 반환
        // (예: RabbitMQ, Kafka, Redis, 인메모리 등)
        Arc::new(InMemoryEventBus::default())
    }
} 