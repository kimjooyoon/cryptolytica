# 거래소 도메인 (Exchange Domain)

거래소 도메인은 CryptoLytica 시스템에서 암호화폐 거래소와의 연동, 통신, 주문 관리를 담당하는 핵심 도메인입니다.

## 책임 영역

거래소 도메인의 주요 책임은 다음과 같습니다:

1. **거래소 모델링**: 다양한 거래소의 특성과 API를 추상화하여 모델링
2. **API 통신**: 거래소 API와의 HTTP, WebSocket 통신 처리
3. **주문 관리**: 거래 주문 생성, 취소, 조회 처리
4. **계정 관리**: 잔고 조회, 거래 내역 관리
5. **인증 처리**: API 키 관리 및 인증 메커니즘 구현
6. **에러 처리**: 거래소별 에러 처리 및 재시도 전략
7. **속도 제한 관리**: API 요청 속도 제한(Rate Limit) 준수

## 핵심 모델

### 거래소 (Exchange)

거래소 정보와 API 연결 설정을 표현하는 모델입니다. 지원하는 기능, 통신 URL, 속도 제한 등의 정보를 포함합니다.

### 주문 (Order)

거래소에 제출된 주문을 표현하는 모델입니다. 주문 유형, 금액, 수량, 주문 상태 등의 정보를 포함합니다.

### 계정 잔고 (Balance)

계정의 자산 잔고를 표현하는 모델입니다. 통화, 가용 잔액, 동결 잔액 등의 정보를 포함합니다.

### 거래 (Trade)

체결된 거래 내역을 표현하는 모델입니다. 거래 시간, 가격, 수량, 수수료 등의 정보를 포함합니다.

### 거래소 인증 정보 (Credentials)

거래소 API 접근에 필요한 인증 정보를 안전하게 관리하는 모델입니다.

## 주요 인터페이스

### 거래소 서비스 인터페이스

```rust
pub trait ExchangeService: Send + Sync {
    // 일반 정보 조회
    async fn get_exchange_info(&self) -> Result<ExchangeInfo>;
    async fn get_symbols(&self) -> Result<Vec<SymbolPair>>;
    
    // 주문 관련
    async fn create_order(&self, order: &OrderRequest) -> Result<Order>;
    async fn cancel_order(&self, symbol: &SymbolPair, order_id: &str) -> Result<Order>;
    async fn get_order(&self, symbol: &SymbolPair, order_id: &str) -> Result<Order>;
    async fn get_open_orders(&self, symbol: Option<&SymbolPair>) -> Result<Vec<Order>>;
    
    // 계정 관련
    async fn get_balances(&self) -> Result<Vec<Balance>>;
    async fn get_trades(&self, symbol: &SymbolPair, limit: Option<u32>) -> Result<Vec<Trade>>;
    
    // WebSocket 관련
    async fn subscribe_to_ticker(&self, symbol: &SymbolPair, callback: Box<dyn TickerCallback>) -> Result<SubscriptionHandle>;
    async fn subscribe_to_trades(&self, symbol: &SymbolPair, callback: Box<dyn TradeCallback>) -> Result<SubscriptionHandle>;
    async fn unsubscribe(&self, handle: SubscriptionHandle) -> Result<()>;
}
```

### 거래소 팩토리 인터페이스

```rust
pub trait ExchangeFactory: Send + Sync {
    fn create_exchange(&self, exchange_id: &ExchangeId, config: ExchangeConfig) -> Result<Box<dyn ExchangeService>>;
    fn supported_exchanges(&self) -> Vec<ExchangeId>;
}
```

## 이벤트

거래소 도메인은 다음과 같은 이벤트를 발행합니다:

- `OrderCreated`: 주문이 생성되었을 때
- `OrderCancelled`: 주문이 취소되었을 때
- `OrderFilled`: 주문이 체결되었을 때
- `TradeExecuted`: 거래가 실행되었을 때
- `BalanceChanged`: 계정 잔고가 변경되었을 때
- `ExchangeError`: 거래소 통신 오류가 발생했을 때

## 의존성

거래소 도메인은 다음과 같은 의존성을 갖습니다:

- `shared-kernel`: 공통 타입, 유틸리티, 에러 처리 등

## 사용 예시

```rust
// 거래소 서비스 사용 예시
async fn example_usage(exchange_factory: &dyn ExchangeFactory) -> Result<()> {
    // 거래소 설정
    let config = ExchangeConfig {
        api_key: Some("your_api_key".to_string()),
        api_secret: Some("your_api_secret".to_string()),
        // 기타 설정
        ..Default::default()
    };
    
    // 거래소 서비스 생성
    let exchange = exchange_factory.create_exchange(&ExchangeId::new("binance"), config)?;
    
    // 심볼 정보 조회
    let symbols = exchange.get_symbols().await?;
    
    // 잔고 조회
    let balances = exchange.get_balances().await?;
    
    // 주문 생성
    let order_request = OrderRequest::new(
        SymbolPair::new("BTC", "USDT"),
        OrderSide::Buy,
        OrderType::Limit,
        0.001,
        Some(50000.0),
    );
    
    let order = exchange.create_order(&order_request).await?;
    println!("주문 생성 완료: {:?}", order);
    
    Ok(())
}
```

## 개발 가이드라인

1. 각 거래소의 구현은 공통 인터페이스를 따라야 하며, 거래소별 특성은 내부적으로 처리해야 합니다.
2. 모든 API 통신은 예외 처리와 재시도 메커니즘을 포함해야 합니다.
3. 속도 제한(Rate Limit)을 준수하여 요청을 관리해야 합니다.
4. 민감한 인증 정보는 항상 안전하게 다루어야 합니다.
5. 비동기 처리를 통해 성능을 최적화해야 합니다.
6. 각 거래소 구현체는 개별적으로 철저히 테스트되어야 합니다. 