// memory_event_bus.rs
//
// 인메모리 이벤트 버스 구현
// 도메인 간 이벤트 통신을 위한 메모리 내 이벤트 버스

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use cryptolytica_shared_kernel::events::{Event, EventBus, EventHandler, SubscriptionHandle};
use cryptolytica_shared_kernel::error::CoreError;
use cryptolytica_shared_kernel::types::Result;

type BoxedHandler<E> = Box<dyn Fn(&E) -> Result<()> + Send + Sync + 'static>;
type AnyBoxedHandler = Box<dyn Any + Send + Sync>;

/// 인메모리 이벤트 버스 구현
#[derive(Default)]
pub struct InMemoryEventBus {
    handlers: RwLock<HashMap<TypeId, Vec<(Uuid, AnyBoxedHandler)>>>,
}

impl InMemoryEventBus {
    /// 새로운 인메모리 이벤트 버스 생성
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
    
    /// 이벤트 타입에 대한 핸들러 가져오기
    fn get_handlers_for_type<E: Event + for<'de> Deserialize<'de>>(
        &self,
    ) -> Vec<(Uuid, BoxedHandler<E>)> {
        let handlers = self.handlers.read().unwrap();
        let type_id = TypeId::of::<E>();
        
        if let Some(type_handlers) = handlers.get(&type_id) {
            type_handlers
                .iter()
                .filter_map(|(id, handler)| {
                    // 핸들러를 E 타입에 대한 함수로 다운캐스트
                    if let Some(typed_handler) = handler.downcast_ref::<BoxedHandler<E>>() {
                        Some((*id, typed_handler.clone()))
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl EventBus for InMemoryEventBus {
    /// 이벤트 발행
    fn publish<E: Event + Serialize>(&self, event: E) -> Result<()> {
        let type_id = TypeId::of::<E>();
        let handlers = self.handlers.read().unwrap();
        
        if let Some(type_handlers) = handlers.get(&type_id) {
            // 각 핸들러에게 이벤트 전달
            for (_, handler) in type_handlers {
                // 핸들러를 E 타입에 대한 함수로 다운캐스트
                if let Some(typed_handler) = handler.downcast_ref::<BoxedHandler<E>>() {
                    // 핸들러 실행
                    if let Err(e) = typed_handler(&event) {
                        tracing::error!("이벤트 핸들러 실행 중 오류: {:?}", e);
                    }
                }
            }
            
            tracing::debug!(
                "이벤트 발행됨: {} (핸들러 {}개)",
                event.event_type(),
                type_handlers.len()
            );
        } else {
            tracing::debug!("이벤트 발행됨: {} (구독자 없음)", event.event_type());
        }
        
        Ok(())
    }
    
    /// 이벤트 구독
    fn subscribe<E: Event + for<'de> Deserialize<'de>, H: EventHandler<E>>(
        &self,
        handler: H,
    ) -> Result<SubscriptionHandle> {
        let type_id = TypeId::of::<E>();
        let subscription_id = Uuid::new_v4();
        
        // 클로저로 핸들러 래핑
        let handler = Arc::new(handler);
        let boxed_handler: BoxedHandler<E> = Box::new(move |event: &E| {
            handler.handle(event)
        });
        
        // Any로 변환하여 저장
        let any_handler: AnyBoxedHandler = Box::new(boxed_handler);
        
        // 핸들러 맵에 추가
        let mut handlers = self.handlers.write().unwrap();
        handlers
            .entry(type_id)
            .or_insert_with(Vec::new)
            .push((subscription_id, any_handler));
        
        tracing::debug!("이벤트 구독 등록됨: {:?}", type_id);
        
        // 구독 핸들 생성 및 반환
        Ok(SubscriptionHandle::new::<E>(subscription_id))
    }
    
    /// 구독 취소
    fn unsubscribe(&self, handle: &SubscriptionHandle) -> Result<()> {
        let mut handlers = self.handlers.write().unwrap();
        
        if let Some(type_handlers) = handlers.get_mut(&handle.event_type_id) {
            // 구독 ID에 해당하는 핸들러 제거
            let original_len = type_handlers.len();
            type_handlers.retain(|(id, _)| *id != handle.id);
            
            if original_len != type_handlers.len() {
                tracing::debug!("이벤트 구독 취소됨: {:?}", handle.event_type_id);
                return Ok(());
            }
        }
        
        Err(CoreError::NotFound(format!(
            "구독 핸들을 찾을 수 없음: {}",
            handle.id
        )).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
    use cryptolytica_shared_kernel::events::Event;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct TestEvent {
        id: Uuid,
        message: String,
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
    
    struct TestHandler {
        received: Arc<RwLock<Vec<String>>>,
    }
    
    impl EventHandler<TestEvent> for TestHandler {
        fn handle(&self, event: &TestEvent) -> Result<()> {
            let mut received = self.received.write().unwrap();
            received.push(event.message.clone());
            Ok(())
        }
    }
    
    #[test]
    fn test_publish_subscribe() {
        let event_bus = InMemoryEventBus::new();
        let received = Arc::new(RwLock::new(Vec::new()));
        
        let handler = TestHandler {
            received: received.clone(),
        };
        
        // 구독
        let handle = event_bus.subscribe(handler).unwrap();
        
        // 이벤트 발행
        let event = TestEvent {
            id: Uuid::new_v4(),
            message: "Hello, World!".to_string(),
            timestamp: Utc::now(),
        };
        
        event_bus.publish(event).unwrap();
        
        // 결과 확인
        let received = received.read().unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0], "Hello, World!");
        
        // 구독 취소
        event_bus.unsubscribe(&handle).unwrap();
        
        // 다시 이벤트 발행
        let event = TestEvent {
            id: Uuid::new_v4(),
            message: "After unsubscribe".to_string(),
            timestamp: Utc::now(),
        };
        
        event_bus.publish(event).unwrap();
        
        // 구독 취소 후에는 이벤트가 처리되지 않아야 함
        let received = received.read().unwrap();
        assert_eq!(received.len(), 1);
    }
} 