# CryptoLytica 도메인 모델 및 이벤트

## 도메인 이벤트 정의

도메인 이벤트는 시스템 내에서 발생하는 중요한 상태 변화를 나타냅니다. CryptoLytica에서는 아래와 같은 주요 도메인 이벤트를 정의합니다.

### 1. 거래소 컨텍스트 (Exchange Context)

#### 1.1 거래소 연결 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `ExchangeConnected` | 거래소 API 연결 성공 | exchangeId, timestamp, connectionInfo |
| `ExchangeDisconnected` | 거래소 API 연결 해제 | exchangeId, timestamp, reason |
| `ExchangeConnectionFailed` | 거래소 API 연결 실패 | exchangeId, timestamp, errorDetails, retryCount |
| `ExchangeAuthenticationSucceeded` | 인증 성공 | exchangeId, timestamp, authType |
| `ExchangeAuthenticationFailed` | 인증 실패 | exchangeId, timestamp, authType, errorDetails |
| `ExchangeRateLimitReached` | API 호출 제한 도달 | exchangeId, timestamp, limitType, resetTime |

#### 1.2 거래소 구성 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `ExchangeAdded` | 새 거래소 추가 | exchangeId, exchangeName, supportedFeatures, timestamp |
| `ExchangeUpdated` | 거래소 설정 변경 | exchangeId, changedProperties, timestamp |
| `ExchangeRemoved` | 거래소 제거 | exchangeId, reason, timestamp |
| `ExchangeFeatureToggled` | 거래소 기능 활성화/비활성화 | exchangeId, feature, enabled, timestamp |

### 2. 시장 데이터 컨텍스트 (Market Data Context)

#### 2.1 데이터 수집 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `MarketDataReceived` | 새 시장 데이터 수신 | exchangeId, symbol, dataType, timestamp, data |
| `OHLCVDataReceived` | OHLCV 데이터 수신 | exchangeId, symbol, timeframe, timestamp, data |
| `OrderBookReceived` | 오더북 데이터 수신 | exchangeId, symbol, timestamp, asks, bids, depth |
| `TradeDataReceived` | 거래 데이터 수신 | exchangeId, symbol, timestamp, trades |
| `DataCollectionStarted` | 데이터 수집 시작 | exchangeId, symbols, dataTypes, timestamp |
| `DataCollectionStopped` | 데이터 수집 중지 | exchangeId, symbols, dataTypes, timestamp, reason |
| `DataCollectionFailed` | 데이터 수집 실패 | exchangeId, symbols, dataTypes, timestamp, errorDetails |

#### 2.2 데이터 처리 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `MarketDataNormalized` | 데이터 정규화 완료 | sourceExchangeId, symbol, dataType, timestamp, normalizedData |
| `MarketDataAggregated` | 데이터 집계 완료 | symbols, dataType, timeframe, timestamp, aggregatedData |
| `AnomalyDetected` | 이상 데이터 감지 | exchangeId, symbol, dataType, timestamp, anomalyDetails |
| `MarketDataValidated` | 데이터 유효성 검증 완료 | dataId, validationResults, timestamp |

#### 2.3 데이터 저장 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `MarketDataStored` | 데이터 저장 완료 | dataId, dataType, timestamp, storageDetails |
| `MarketDataFetchRequested` | 데이터 조회 요청 | requestId, filters, timestamp |
| `MarketDataFetched` | 데이터 조회 완료 | requestId, dataCount, timestamp |
| `DataRetentionPolicyApplied` | 데이터 보존 정책 적용 | dataType, affectedRecords, timestamp |

### 3. 분석 컨텍스트 (Analytics Context)

#### 3.1 분석 작업 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `AnalysisJobCreated` | 분석 작업 생성 | jobId, analysisType, parameters, timestamp |
| `AnalysisJobStarted` | 분석 작업 시작 | jobId, timestamp |
| `AnalysisJobCompleted` | 분석 작업 완료 | jobId, resultSummary, timestamp |
| `AnalysisJobFailed` | 분석 작업 실패 | jobId, errorDetails, timestamp |
| `AnalysisResultsAvailable` | 분석 결과 이용 가능 | jobId, resultLocation, timestamp |

#### 3.2 분석 모델 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `ModelTrainingStarted` | 모델 학습 시작 | modelId, modelType, datasetInfo, timestamp |
| `ModelTrainingCompleted` | 모델 학습 완료 | modelId, metrics, timestamp |
| `ModelTrainingFailed` | 모델 학습 실패 | modelId, errorDetails, timestamp |
| `ModelDeployed` | 모델 배포 완료 | modelId, deploymentInfo, timestamp |
| `ModelPredictionGenerated` | 모델 예측 생성 | modelId, predictionId, inputParameters, timestamp, predictions |

#### 3.3 시장 패턴 관련 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `PatternDetected` | 시장 패턴 감지 | patternId, patternType, symbol, timeframe, confidence, timestamp |
| `VolatilityAnalysisCompleted` | 변동성 분석 완료 | symbol, timeframe, volatilityMetrics, timestamp |
| `CorrelationAnalysisCompleted` | 자산 간 상관관계 분석 완료 | symbols, correlationMatrix, timestamp |
| `TrendChangeDetected` | 추세 변화 감지 | symbol, oldTrend, newTrend, confidence, timestamp |

### 4. 포트폴리오 컨텍스트 (Portfolio Context)

#### 4.1 포트폴리오 관리 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `PortfolioCreated` | 포트폴리오 생성 | portfolioId, name, baseAsset, timestamp |
| `PortfolioUpdated` | 포트폴리오 정보 업데이트 | portfolioId, changedProperties, timestamp |
| `PortfolioDeleted` | 포트폴리오 삭제 | portfolioId, timestamp |
| `AssetAddedToPortfolio` | 자산 추가 | portfolioId, assetId, amount, timestamp |
| `AssetRemovedFromPortfolio` | 자산 제거 | portfolioId, assetId, timestamp |

#### 4.2 포트폴리오 분석 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `PortfolioPerformanceCalculated` | 포트폴리오 성과 계산 | portfolioId, period, metrics, timestamp |
| `PortfolioRebalanced` | 포트폴리오 리밸런싱 수행 | portfolioId, transactions, newWeights, timestamp |
| `OptimalAllocationCalculated` | 최적 자산 배분 계산 | portfolioId, allocationStrategy, optimizedWeights, timestamp |
| `RiskAnalysisCompleted` | 리스크 분석 완료 | portfolioId, riskMetrics, timestamp |

### 5. 알림 컨텍스트 (Notification Context)

#### 5.1 알림 관리 이벤트

| 이벤트 | 설명 | 속성 |
|--------|------|------|
| `AlertConfigured` | 알림 구성 | alertId, alertType, conditions, timestamp |
| `AlertTriggered` | 알림 조건 충족 | alertId, triggerValue, timestamp |
| `AlertDelivered` | 알림 전달 완료 | alertId, deliveryChannel, timestamp |
| `AlertDeliveryFailed` | 알림 전달 실패 | alertId, deliveryChannel, errorDetails, timestamp |

## 주요 애그리게이트 및 엔티티

### 1. Exchange 애그리게이트 (거래소)

거래소 API 연결 및 관리

**애그리게이트 루트**: Exchange

**엔티티**:
- Exchange: 거래소 정보
- ApiCredential: API 인증 정보 
- RateLimitStatus: API 사용량 상태

**값 객체**:
- ExchangeFeature: 거래소 지원 기능
- ConnectionConfig: 연결 설정

### 2. MarketData 애그리게이트 (시장 데이터)

시장 데이터 수집 및 저장

**애그리게이트 루트**: MarketDataStream

**엔티티**:
- MarketDataStream: 데이터 스트림 정보
- OHLCV: 가격 캔들스틱 데이터
- OrderBook: 호가창 데이터
- Trade: 체결 내역 데이터

**값 객체**:
- Symbol: 거래 쌍 정보
- Timeframe: 시간 프레임
- Price: 가격 정보

### 3. Analysis 애그리게이트 (분석)

데이터 분석 작업

**애그리게이트 루트**: AnalysisJob

**엔티티**:
- AnalysisJob: 분석 작업
- AnalysisModel: 분석 모델
- AnalysisResult: 분석 결과

**값 객체**:
- AnalysisParameters: 분석 매개변수
- PatternInfo: 패턴 정보
- ModelMetrics: 모델 측정 지표

### 4. Portfolio 애그리게이트 (포트폴리오)

포트폴리오 관리 및 최적화

**애그리게이트 루트**: Portfolio

**엔티티**:
- Portfolio: 포트폴리오 정보
- Asset: 자산 정보
- Transaction: 거래 내역

**값 객체**:
- AssetAllocation: 자산 배분 정보
- PerformanceMetrics: 성과 지표
- RiskMetrics: 리스크 지표

### 5. Alert 애그리게이트 (알림)

알림 구성 및 발송

**애그리게이트 루트**: Alert

**엔티티**:
- Alert: 알림 정보
- AlertHistory: 알림 발송 이력

**값 객체**:
- AlertCondition: 알림 조건
- AlertChannel: 알림 채널 정보

## 바운디드 컨텍스트 간 관계

1. **Exchange ↔ MarketData**: Customer-Supplier 관계
   - Exchange 컨텍스트는 MarketData 컨텍스트에 데이터 제공

2. **MarketData ↔ Analysis**: Customer-Supplier 관계
   - MarketData 컨텍스트는 Analysis 컨텍스트에 데이터 제공

3. **Analysis ↔ Portfolio**: Partnership 관계
   - 상호 협력하여 포트폴리오 최적화 및 분석 제공

4. **Analysis ↔ Alert**: Customer-Supplier 관계
   - Analysis 컨텍스트는 Alert 컨텍스트에 알림 조건 충족 정보 제공

5. **Portfolio ↔ Alert**: Customer-Supplier 관계
   - Portfolio 컨텍스트는 Alert 컨텍스트에 포트폴리오 관련 알림 조건 제공

## 공유 커널 (Shared Kernel)

다음 개념은 모든 컨텍스트에서 일관되게 사용됩니다:

- Asset: 암호화폐 자산 정보
- Symbol: 거래 쌍 표기
- Timestamp: 시간 정보
- Price: 가격 정보