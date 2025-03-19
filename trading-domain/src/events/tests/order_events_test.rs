use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use rust_decimal_macros::dec;

use cryptolytica_shared_kernel::types::{OrderId, OrderSide, OrderStatus, OrderType, SymbolPair};
use cryptolytica_shared_kernel::events::EventHandler;
use crate::model::exchange_view::order::OrderCache;
use crate::events::exchange_events::order_events::{
    OrderCreatedEvent, OrderCreatedEventHandler,
    OrderStatusUpdatedEvent, OrderStatusUpdatedEventHandler,
    OrderFilledEvent, OrderFilledEventHandler
};

#[test]
fn test_order_created_event_handler() {
    // 설정
    let order_cache = Arc::new(OrderCache::new());
    let handler = OrderCreatedEventHandler::new(order_cache.clone());
    
    let order_id = OrderId::new();
    let exchange = "Binance".to_string();
    let symbol_pair = SymbolPair::new("BTC", "USDT");
    let order_type = OrderType::Limit;
    let side = OrderSide::Buy;
    let quantity = dec!(1);
    let price = Some(dec!(50000));
    let client_order_id = Some("client123".to_string());
    
    let event = OrderCreatedEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        order_type,
        side,
        quantity,
        price,
        client_order_id: client_order_id.clone(),
    };
    
    // 실행
    let result = handler.handle(&event);
    
    // 검증
    assert!(result.is_ok());
    let order = order_cache.get_order(&order_id);
    assert!(order.is_some());
    
    let order = order.unwrap();
    assert_eq!(order.order_id, order_id);
    assert_eq!(order.exchange, exchange);
    assert_eq!(order.symbol_pair, symbol_pair);
    assert_eq!(order.order_type, order_type);
    assert_eq!(order.side, side);
    assert_eq!(order.quantity, quantity);
    assert_eq!(order.price, price);
    assert_eq!(order.client_order_id, client_order_id);
    assert_eq!(order.status, OrderStatus::New); // 기본 상태
}

#[test]
fn test_order_status_updated_event_handler() {
    // 설정: 먼저 주문 생성
    let order_cache = Arc::new(OrderCache::new());
    let created_handler = OrderCreatedEventHandler::new(order_cache.clone());
    let status_handler = OrderStatusUpdatedEventHandler::new(order_cache.clone());
    
    let order_id = OrderId::new();
    let exchange = "Binance".to_string();
    let symbol_pair = SymbolPair::new("BTC", "USDT");
    
    // 주문 생성 이벤트
    let create_event = OrderCreatedEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        order_type: OrderType::Limit,
        side: OrderSide::Buy,
        quantity: dec!(1),
        price: Some(dec!(50000)),
        client_order_id: Some("client123".to_string()),
    };
    
    created_handler.handle(&create_event).unwrap();
    
    // 상태 업데이트 이벤트
    let new_status = OrderStatus::PartiallyFilled;
    let filled_quantity = dec!(0.5);
    let avg_price = dec!(49900);
    
    let status_event = OrderStatusUpdatedEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        status: new_status,
        filled_quantity: Some(filled_quantity),
        average_fill_price: Some(avg_price),
        cancel_reason: None,
    };
    
    // 실행
    let result = status_handler.handle(&status_event);
    
    // 검증
    assert!(result.is_ok());
    let order = order_cache.get_order(&order_id).unwrap();
    assert_eq!(order.status, new_status);
    assert_eq!(order.filled_quantity, Some(filled_quantity));
    assert_eq!(order.average_fill_price, Some(avg_price));
}

#[test]
fn test_order_filled_event_handler() {
    // 설정: 먼저 주문 생성
    let order_cache = Arc::new(OrderCache::new());
    let created_handler = OrderCreatedEventHandler::new(order_cache.clone());
    let filled_handler = OrderFilledEventHandler::new(order_cache.clone());
    
    let order_id = OrderId::new();
    let exchange = "Binance".to_string();
    let symbol_pair = SymbolPair::new("ETH", "USDT");
    let quantity = dec!(5);
    
    // 주문 생성 이벤트
    let create_event = OrderCreatedEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        order_type: OrderType::Market,
        side: OrderSide::Sell,
        quantity,
        price: None, // 시장가라서 가격 없음
        client_order_id: None,
    };
    
    created_handler.handle(&create_event).unwrap();
    
    // 체결 이벤트
    let fill_quantity = dec!(2);
    let fill_price = dec!(2000);
    let total_filled = dec!(2);
    let avg_fill_price = dec!(2000);
    let is_complete = false; // 부분 체결
    
    let fill_event = OrderFilledEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        fill_quantity,
        fill_price,
        total_filled,
        average_fill_price: avg_fill_price,
        fee: Some(dec!(2)),
        fee_asset: Some("USDT".to_string()),
        is_complete,
    };
    
    // 실행
    let result = filled_handler.handle(&fill_event);
    
    // 검증
    assert!(result.is_ok());
    let order = order_cache.get_order(&order_id).unwrap();
    assert_eq!(order.filled_quantity, Some(total_filled));
    assert_eq!(order.average_fill_price, Some(avg_fill_price));
    assert_eq!(order.status, OrderStatus::PartiallyFilled); // 부분 체결로 상태 변경됨
    
    // 완전 체결 테스트
    let fill_quantity2 = dec!(3);
    let fill_price2 = dec!(2010);
    let total_filled2 = dec!(5); // 완전 체결
    let avg_fill_price2 = dec!(2006); // 평균 체결가
    let is_complete2 = true; // 완전 체결
    
    let fill_event2 = OrderFilledEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        order_id: order_id.clone(),
        exchange: exchange.clone(),
        symbol_pair: symbol_pair.clone(),
        fill_quantity: fill_quantity2,
        fill_price: fill_price2,
        total_filled: total_filled2,
        average_fill_price: avg_fill_price2,
        fee: Some(dec!(3)),
        fee_asset: Some("USDT".to_string()),
        is_complete: is_complete2,
    };
    
    // 두 번째 체결 실행
    let result2 = filled_handler.handle(&fill_event2);
    
    // 검증
    assert!(result2.is_ok());
    let order2 = order_cache.get_order(&order_id).unwrap();
    assert_eq!(order2.filled_quantity, Some(total_filled2));
    assert_eq!(order2.average_fill_price, Some(avg_fill_price2));
    assert_eq!(order2.status, OrderStatus::Filled); // 완전 체결로 상태 변경됨
} 