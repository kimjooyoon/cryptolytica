use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use cryptolytica_shared_kernel::events::{Event, EventHandler};
use cryptolytica_shared_kernel::types::Result;
use crate::events::memory_event_bus::InMemoryEventBus;

// 테스트용 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestEvent {
    id: Uuid,
    message: String,
    timestamp: chrono::DateTime<Utc>,
}

impl Event for TestEvent {
    fn event_type(&self) -> &'static str {
        "test.event"
    }
    
    fn timestamp(&self) -> chrono::DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &Uuid {
        &self.id
    }
}

// 다른 타입의 테스트 이벤트
#[derive(Debug, Clone, Serialize, Deserialize)]
struct OtherTestEvent {
    id: Uuid,
    value: i32,
    timestamp: chrono::DateTime<Utc>,
}

impl Event for OtherTestEvent {
    fn event_type(&self) -> &'static str {
        "other.test.event"
    }
    
    fn timestamp(&self) -> chrono::DateTime<Utc> {
        self.timestamp
    }
    
    fn id(&self) -> &Uuid {
        &self.id
    }
}

// 테스트용 핸들러
struct TestHandler {
    counter: Arc<AtomicUsize>,
    last_message: Arc<std::sync::Mutex<Option<String>>>,
}

impl EventHandler<TestEvent> for TestHandler {
    fn handle(&self, event: &TestEvent) -> Result<()> {
        self.counter.fetch_add(1, Ordering::SeqCst);
        let mut last_message = self.last_message.lock().unwrap();
        *last_message = Some(event.message.clone());
        Ok(())
    }
}

struct OtherTestHandler {
    counter: Arc<AtomicUsize>,
}

impl EventHandler<OtherTestEvent> for OtherTestHandler {
    fn handle(&self, event: &OtherTestEvent) -> Result<()> {
        self.counter.fetch_add(event.value as usize, Ordering::SeqCst);
        Ok(())
    }
}

#[test]
fn test_multiple_handlers_for_same_event() {
    // 설정
    let event_bus = InMemoryEventBus::new();
    
    let counter1 = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::new(AtomicUsize::new(0));
    
    let handler1 = TestHandler {
        counter: counter1.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    let handler2 = TestHandler {
        counter: counter2.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    // 두 핸들러 모두 구독
    let _handle1 = event_bus.subscribe(handler1).unwrap();
    let _handle2 = event_bus.subscribe(handler2).unwrap();
    
    // 이벤트 발행
    let event = TestEvent {
        id: Uuid::new_v4(),
        message: "Hello, World!".to_string(),
        timestamp: Utc::now(),
    };
    
    event_bus.publish(event).unwrap();
    
    // 두 핸들러 모두 이벤트를 처리했는지 확인
    assert_eq!(counter1.load(Ordering::SeqCst), 1);
    assert_eq!(counter2.load(Ordering::SeqCst), 1);
}

#[test]
fn test_different_event_types() {
    // 설정
    let event_bus = InMemoryEventBus::new();
    
    let test_counter = Arc::new(AtomicUsize::new(0));
    let other_counter = Arc::new(AtomicUsize::new(0));
    
    let test_handler = TestHandler {
        counter: test_counter.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    let other_handler = OtherTestHandler {
        counter: other_counter.clone(),
    };
    
    // 다른 타입의 이벤트 핸들러 구독
    let _test_handle = event_bus.subscribe(test_handler).unwrap();
    let _other_handle = event_bus.subscribe(other_handler).unwrap();
    
    // TestEvent 발행
    let test_event = TestEvent {
        id: Uuid::new_v4(),
        message: "Test Event".to_string(),
        timestamp: Utc::now(),
    };
    
    event_bus.publish(test_event).unwrap();
    
    // OtherTestEvent 발행
    let other_event = OtherTestEvent {
        id: Uuid::new_v4(),
        value: 42,
        timestamp: Utc::now(),
    };
    
    event_bus.publish(other_event).unwrap();
    
    // 각 핸들러가 자신의 타입의 이벤트만 받았는지 확인
    assert_eq!(test_counter.load(Ordering::SeqCst), 1);
    assert_eq!(other_counter.load(Ordering::SeqCst), 42);
}

#[test]
fn test_handler_error_handling() {
    // 설정
    let event_bus = InMemoryEventBus::new();
    
    // 항상 에러를 반환하는 핸들러
    struct ErrorHandler;
    
    impl EventHandler<TestEvent> for ErrorHandler {
        fn handle(&self, _event: &TestEvent) -> Result<()> {
            Err("핸들러 오류".into())
        }
    }
    
    // 정상 동작하는 핸들러
    let counter = Arc::new(AtomicUsize::new(0));
    let normal_handler = TestHandler {
        counter: counter.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    // 두 핸들러 모두 구독
    let _error_handle = event_bus.subscribe(ErrorHandler).unwrap();
    let _normal_handle = event_bus.subscribe(normal_handler).unwrap();
    
    // 이벤트 발행
    let event = TestEvent {
        id: Uuid::new_v4(),
        message: "Error Test".to_string(),
        timestamp: Utc::now(),
    };
    
    // 핸들러 에러가 있어도 publish 자체는 성공해야 함
    let result = event_bus.publish(event);
    assert!(result.is_ok());
    
    // 정상 핸들러는 여전히 실행되어야 함
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[test]
fn test_unsubscribe_specific_handler() {
    // 설정
    let event_bus = InMemoryEventBus::new();
    
    let counter1 = Arc::new(AtomicUsize::new(0));
    let counter2 = Arc::new(AtomicUsize::new(0));
    
    let handler1 = TestHandler {
        counter: counter1.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    let handler2 = TestHandler {
        counter: counter2.clone(),
        last_message: Arc::new(std::sync::Mutex::new(None)),
    };
    
    // 두 핸들러 모두 구독
    let handle1 = event_bus.subscribe(handler1).unwrap();
    let _handle2 = event_bus.subscribe(handler2).unwrap();
    
    // 첫 번째 핸들러 구독 취소
    let result = event_bus.unsubscribe(&handle1);
    assert!(result.is_ok());
    
    // 이벤트 발행
    let event = TestEvent {
        id: Uuid::new_v4(),
        message: "After unsubscribe".to_string(),
        timestamp: Utc::now(),
    };
    
    event_bus.publish(event).unwrap();
    
    // 두 번째 핸들러만 이벤트를 처리해야 함
    assert_eq!(counter1.load(Ordering::SeqCst), 0);
    assert_eq!(counter2.load(Ordering::SeqCst), 1);
} 