# 시장 데이터 도메인 (Market Domain)

시장 데이터 도메인은 CryptoLytica 시스템에서 암호화폐 시장 데이터의 수집, 저장, 처리, 조회 관련 책임을 갖는 핵심 도메인입니다.

## 책임 영역

시장 데이터 도메인의 주요 책임은 다음과 같습니다:

1. **시장 데이터 모델링**: 캔들스틱, 오더북, 거래 내역 등의 데이터 모델 정의
2. **데이터 정규화**: 다양한 거래소에서 수집된 데이터의 표준화 및 정규화
3. **저장소 인터페이스**: 시장 데이터 저장 및 조회를 위한 리포지토리 인터페이스 정의
4. **데이터 서비스**: 시장 데이터 조회, 집계, 분석 등을 위한 서비스 인터페이스 제공
5. **도메인 이벤트**: 시장 데이터 관련 이벤트 정의 및 발행

## 핵심 모델

### 캔들 (Candle)

OHLCV(시가, 고가, 저가, 종가, 거래량) 데이터를 표현하는 모델입니다. 다양한 타임프레임(1분, 5분, 1시간 등)을 지원합니다.

### 오더북 (OrderBook)

거래소의 매수/매도 주문 스냅샷을 표현하는 모델입니다. 가격과 수량 정보를 포함합니다.

### 티커 (Ticker)

실시간 가격 정보를 표현하는 모델입니다. 최신 가격, 24시간 변동률 등의 정보를 포함합니다.

### 거래 (Trade)

개별 거래 내역을 표현하는 모델입니다. 거래 시간, 가격, 수량, 방향 등의 정보를 포함합니다.

### 시장 데이터 (MarketData)

다양한 유형의 시장 데이터를 통합적으로 관리하기 위한 메타 모델입니다.

## 주요 인터페이스

### 저장소 인터페이스

```rust
pub trait MarketDataRepository: Send + Sync {
    async fn save_candle(&self, candle: &Candle) -> Result<()>;
    async fn save_candles(&self, candles: &[Candle]) -> Result<()>;
    async fn get_candles(&self, symbol: &SymbolPair, timeframe: Timeframe, 
                         from: DateTime<Utc>, to: DateTime<Utc>) -> Result<Vec<Candle>>;
    // ... 기타 메서드
}
```

### 서비스 인터페이스

```rust
pub trait MarketDataService: Send + Sync {
    async fn get_latest_price(&self, symbol: &SymbolPair) -> Result<f64>;
    async fn get_price_history(&self, symbol: &SymbolPair, timeframe: Timeframe, 
                              limit: usize) -> Result<Vec<Candle>>;
    async fn get_volatility(&self, symbol: &SymbolPair, period: usize) -> Result<f64>;
    // ... 기타 메서드
}
```

## 이벤트

시장 데이터 도메인은 다음과 같은 이벤트를 발행합니다:

- `CandleCreated`: 새로운 캔들이 생성되었을 때
- `CandleUpdated`: 기존 캔들이 업데이트되었을 때
- `PriceChanged`: 가격이 변동되었을 때
- `VolatilityThresholdCrossed`: 변동성이 임계값을 넘었을 때

## 의존성

시장 데이터 도메인은 다음과 같은 의존성을 갖습니다:

- `shared-kernel`: 공통 타입, 유틸리티, 에러 처리 등

## 사용 예시

```rust
// 서비스 사용 예시
async fn example_usage(market_service: &dyn MarketDataService) -> Result<()> {
    let symbol = SymbolPair::new("BTC", "USDT");
    
    // 최신 가격 조회
    let latest_price = market_service.get_latest_price(&symbol).await?;
    
    // 최근 100개 일봉 데이터 조회
    let candles = market_service.get_price_history(&symbol, Timeframe::Day1, 100).await?;
    
    // 30일 변동성 계산
    let volatility = market_service.get_volatility(&symbol, 30).await?;
    
    Ok(())
}
```

## 개발 가이드라인

1. 도메인 모델은 인프라스트럭처 관심사로부터 독립적이어야 합니다.
2. 저장소 및 서비스 인터페이스는 추상화 수준을 유지하여 구현 상세를 숨겨야 합니다.
3. 모든 공개 API는 명확한 문서화를 포함해야 합니다.
4. 이벤트 기반 통신을 선호하여 다른 도메인과의 결합도를 낮춰야 합니다.
5. 모든 도메인 로직은 단위 테스트로 검증되어야 합니다. 