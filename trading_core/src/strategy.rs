//! 트레이딩 전략 정의 및 구현
//!
//! 이 모듈은 다양한 트레이딩 전략 인터페이스와 공통 구현을 제공합니다.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use cryptolytica_common_core::types::{SymbolPair, Timeframe, Candle};
use cryptolytica_exchange_core::models::OrderSide;
use crate::error::Result;
use crate::models::{Signal, Position, TradeDecision, SignalStrength, StrategyType, StrategyState};

/// 트레이딩 전략에 필요한 기본 인터페이스
#[async_trait]
pub trait Strategy: Send + Sync {
    /// 전략 고유 ID 반환
    fn id(&self) -> Uuid;
    
    /// 전략 이름 반환
    fn name(&self) -> &str;
    
    /// 전략 타입 반환
    fn strategy_type(&self) -> StrategyType;
    
    /// 전략에서 사용하는 타임프레임 반환
    fn timeframe(&self) -> Timeframe;
    
    /// 전략 매개변수 반환
    fn parameters(&self) -> StrategyParameters;
    
    /// 전략 상태 반환
    fn state(&self) -> StrategyState;
    
    /// 전략 상태 설정
    fn set_state(&mut self, state: StrategyState) -> Result<()>;
    
    /// 새로운 캔들 데이터로 전략 업데이트하고 신호 생성
    async fn update(&mut self, candle: &Candle) -> Result<Option<Signal>>;
    
    /// 신호 생성
    async fn generate_signal(&self, candles: &[Candle]) -> Result<Option<Signal>>;
    
    /// 포지션 진입 결정
    async fn decide_entry(&self, signal: &Signal, position: Option<&Position>) -> Result<Option<TradeDecision>>;
    
    /// 포지션 종료 결정
    async fn decide_exit(&self, position: &Position, candle: &Candle) -> Result<Option<TradeDecision>>;
    
    /// 전략 초기화
    async fn initialize(&mut self) -> Result<()>;
    
    /// 전략 정보 직렬화
    fn serialize(&self) -> Result<String>;
    
    /// 전략 중지
    async fn shutdown(&mut self) -> Result<()>;
}

/// 트레이딩 전략 매개변수
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyParameters {
    /// 심볼(거래 쌍)
    pub symbol: SymbolPair,
    /// 지표 설정
    pub indicators: serde_json::Value,
    /// 포지션 사이즈 (0.0 ~ 1.0 사이의 비율)
    pub position_size: f64,
    /// 스톱로스 비율 (0.0 ~ 1.0 사이의 비율)
    pub stop_loss: Option<f64>,
    /// 테이크프로핏 비율 (0.0 ~ 1.0 사이의 비율)
    pub take_profit: Option<f64>,
    /// 최대 동시 포지션 수
    pub max_positions: u32,
    /// 기타 설정
    pub options: serde_json::Value,
}

/// 이동평균 교차 전략 구현
pub struct MovingAverageCrossoverStrategy {
    id: Uuid,
    name: String,
    params: StrategyParameters,
    state: StrategyState,
    fast_period: u32,
    slow_period: u32,
    fast_ma: Vec<f64>,
    slow_ma: Vec<f64>,
    last_updated: Option<DateTime<Utc>>,
}

impl MovingAverageCrossoverStrategy {
    pub fn new(
        name: impl Into<String>,
        symbol: SymbolPair,
        fast_period: u32,
        slow_period: u32,
        position_size: f64,
        stop_loss: Option<f64>,
        take_profit: Option<f64>,
    ) -> Self {
        let indicators = serde_json::json!({
            "fast_period": fast_period,
            "slow_period": slow_period,
        });
        
        let params = StrategyParameters {
            symbol,
            indicators,
            position_size,
            stop_loss,
            take_profit,
            max_positions: 1,
            options: serde_json::json!({}),
        };
        
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            params,
            state: StrategyState::Initialized,
            fast_period,
            slow_period,
            fast_ma: Vec::new(),
            slow_ma: Vec::new(),
            last_updated: None,
        }
    }
    
    /// 이동평균 계산
    fn calculate_ma(&self, prices: &[f64], period: usize) -> Vec<f64> {
        if prices.len() < period {
            return Vec::new();
        }
        
        let mut result = Vec::with_capacity(prices.len() - period + 1);
        
        for i in 0..=prices.len() - period {
            let sum: f64 = prices[i..i+period].iter().sum();
            result.push(sum / period as f64);
        }
        
        result
    }
    
    /// 신호 생성 로직
    fn check_crossover(&self) -> Option<Signal> {
        if self.fast_ma.len() < 2 || self.slow_ma.len() < 2 {
            return None;
        }
        
        let fast_prev = self.fast_ma[self.fast_ma.len() - 2];
        let fast_curr = self.fast_ma[self.fast_ma.len() - 1];
        let slow_prev = self.slow_ma[self.slow_ma.len() - 2];
        let slow_curr = self.slow_ma[self.slow_ma.len() - 1];
        
        // 골든 크로스 (빠른 이평선이 느린 이평선을 상향 돌파)
        if fast_prev <= slow_prev && fast_curr > slow_curr {
            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                symbol: self.params.symbol.clone(),
                timestamp: Utc::now(),
                side: OrderSide::Buy,
                strength: SignalStrength::Strong,
                price: None,
                expiration: None,
                metadata: serde_json::json!({
                    "crossover_type": "golden",
                    "fast_ma": fast_curr,
                    "slow_ma": slow_curr,
                }),
            });
        }
        
        // 데드 크로스 (빠른 이평선이 느린 이평선을 하향 돌파)
        if fast_prev >= slow_prev && fast_curr < slow_curr {
            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                symbol: self.params.symbol.clone(),
                timestamp: Utc::now(),
                side: OrderSide::Sell,
                strength: SignalStrength::Strong,
                price: None,
                expiration: None,
                metadata: serde_json::json!({
                    "crossover_type": "death",
                    "fast_ma": fast_curr,
                    "slow_ma": slow_curr,
                }),
            });
        }
        
        None
    }
}

#[async_trait]
impl Strategy for MovingAverageCrossoverStrategy {
    fn id(&self) -> Uuid {
        self.id
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn strategy_type(&self) -> StrategyType {
        StrategyType::Trend
    }
    
    fn timeframe(&self) -> Timeframe {
        Timeframe::Hour1 // 기본값, 실제로는 설정 가능하게 해야 함
    }
    
    fn parameters(&self) -> StrategyParameters {
        self.params.clone()
    }
    
    fn state(&self) -> StrategyState {
        self.state
    }
    
    fn set_state(&mut self, state: StrategyState) -> Result<()> {
        self.state = state;
        Ok(())
    }
    
    async fn update(&mut self, candle: &Candle) -> Result<Option<Signal>> {
        if candle.symbol != self.params.symbol {
            return Err(crate::error::TradingError::StrategyError(
                format!("심볼 불일치: 예상 {}, 받음 {}", 
                    self.params.symbol.to_string(), 
                    candle.symbol.to_string()
                )
            ));
        }
        
        // 이동평균 계산을 위해 종가만 추출하여 저장
        let close_price = candle.close;
        
        // 내부 데이터에 새 종가 추가
        let mut prices = Vec::new();
        prices.push(close_price);
        
        // 새 이동평균 계산
        self.fast_ma = self.calculate_ma(&prices, self.fast_period as usize);
        self.slow_ma = self.calculate_ma(&prices, self.slow_period as usize);
        
        // 신호 확인
        self.last_updated = Some(candle.timestamp);
        self.check_crossover()
    }
    
    async fn generate_signal(&self, candles: &[Candle]) -> Result<Option<Signal>> {
        if candles.is_empty() {
            return Err(crate::error::TradingError::InsufficientDataError(
                "캔들 데이터가 없습니다".to_string()
            ));
        }
        
        if candles[0].symbol != self.params.symbol {
            return Err(crate::error::TradingError::StrategyError(
                format!("심볼 불일치: 예상 {}, 받음 {}", 
                    self.params.symbol.to_string(), 
                    candles[0].symbol.to_string()
                )
            ));
        }
        
        // 이동평균 계산을 위해 종가만 추출
        let prices: Vec<f64> = candles.iter().map(|c| c.close).collect();
        
        if prices.len() < self.slow_period as usize {
            return Err(crate::error::TradingError::InsufficientDataError(
                format!("이동평균 계산을 위한 데이터가 부족합니다. 필요: {}, 보유: {}", 
                    self.slow_period, 
                    prices.len()
                )
            ));
        }
        
        // 이동평균 계산
        let fast_ma = self.calculate_ma(&prices, self.fast_period as usize);
        let slow_ma = self.calculate_ma(&prices, self.slow_period as usize);
        
        if fast_ma.len() < 2 || slow_ma.len() < 2 {
            return Ok(None);
        }
        
        // 가장 최근 두 값으로 크로스오버 확인
        let fast_prev = fast_ma[fast_ma.len() - 2];
        let fast_curr = fast_ma[fast_ma.len() - 1];
        let slow_prev = slow_ma[slow_ma.len() - 2];
        let slow_curr = slow_ma[slow_ma.len() - 1];
        
        // 골든 크로스 (빠른 이평선이 느린 이평선을 상향 돌파)
        if fast_prev <= slow_prev && fast_curr > slow_curr {
            return Ok(Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                symbol: self.params.symbol.clone(),
                timestamp: candles.last().unwrap().timestamp,
                side: OrderSide::Buy,
                strength: SignalStrength::Strong,
                price: Some(candles.last().unwrap().close),
                expiration: None,
                metadata: serde_json::json!({
                    "crossover_type": "golden",
                    "fast_ma": fast_curr,
                    "slow_ma": slow_curr,
                }),
            }));
        }
        
        // 데드 크로스 (빠른 이평선이 느린 이평선을 하향 돌파)
        if fast_prev >= slow_prev && fast_curr < slow_curr {
            return Ok(Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                symbol: self.params.symbol.clone(),
                timestamp: candles.last().unwrap().timestamp,
                side: OrderSide::Sell,
                strength: SignalStrength::Strong,
                price: Some(candles.last().unwrap().close),
                expiration: None,
                metadata: serde_json::json!({
                    "crossover_type": "death",
                    "fast_ma": fast_curr,
                    "slow_ma": slow_curr,
                }),
            }));
        }
        
        Ok(None)
    }
    
    async fn decide_entry(&self, signal: &Signal, position: Option<&Position>) -> Result<Option<TradeDecision>> {
        // 이미 포지션이 있으면 새 진입 안함
        if position.is_some() {
            return Ok(None);
        }
        
        // 신호가 충분히 강하면 진입
        if signal.strength >= SignalStrength::Medium {
            let decision = TradeDecision {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                signal_id: signal.id,
                timestamp: Utc::now(),
                symbol: signal.symbol.clone(),
                side: signal.side,
                action: crate::models::TradeAction::Enter,
                price: signal.price,
                quantity: None, // 실행기에서 계산
                reason: format!("{} 크로스오버 감지", 
                    if signal.side == OrderSide::Buy { "골든" } else { "데드" }
                ),
                stop_loss: self.params.stop_loss,
                take_profit: self.params.take_profit,
                metadata: signal.metadata.clone(),
            };
            
            return Ok(Some(decision));
        }
        
        Ok(None)
    }
    
    async fn decide_exit(&self, position: &Position, candle: &Candle) -> Result<Option<TradeDecision>> {
        // 직접적인 손절/익절 로직 대신, 반대 신호가 나오면 청산하는 로직
        
        // 종가로 이동평균 업데이트 계산
        let prices = vec![candle.close];
        let fast_ma = self.calculate_ma(&prices, self.fast_period as usize);
        let slow_ma = self.calculate_ma(&prices, self.slow_period as usize);
        
        if fast_ma.is_empty() || slow_ma.is_empty() {
            return Ok(None);
        }
        
        let exit_signal = match position.side {
            // 롱 포지션에서 데드 크로스 발생 시 청산
            OrderSide::Buy if fast_ma[0] < slow_ma[0] => true,
            
            // 숏 포지션에서 골든 크로스 발생 시 청산
            OrderSide::Sell if fast_ma[0] > slow_ma[0] => true,
            
            _ => false,
        };
        
        if exit_signal {
            let decision = TradeDecision {
                id: Uuid::new_v4(),
                strategy_id: self.id,
                signal_id: position.signal_id,
                timestamp: Utc::now(),
                symbol: position.symbol.clone(),
                side: position.side.clone(),
                action: crate::models::TradeAction::Exit,
                price: Some(candle.close),
                quantity: Some(position.quantity),
                reason: "반대 크로스오버 신호 발생".to_string(),
                stop_loss: None,
                take_profit: None,
                metadata: serde_json::json!({
                    "exit_type": "signal_reversal",
                    "position_held_time": position.entry_time
                        .map(|t| (Utc::now() - t).num_seconds())
                        .unwrap_or(0),
                }),
            };
            
            return Ok(Some(decision));
        }
        
        Ok(None)
    }
    
    async fn initialize(&mut self) -> Result<()> {
        self.fast_ma.clear();
        self.slow_ma.clear();
        self.last_updated = None;
        self.state = StrategyState::Running;
        Ok(())
    }
    
    fn serialize(&self) -> Result<String> {
        let serialized = serde_json::to_string(&serde_json::json!({
            "id": self.id.to_string(),
            "name": self.name,
            "type": "moving_average_crossover",
            "parameters": {
                "symbol": self.params.symbol.to_string(),
                "fast_period": self.fast_period,
                "slow_period": self.slow_period,
                "position_size": self.params.position_size,
                "stop_loss": self.params.stop_loss,
                "take_profit": self.params.take_profit,
                "max_positions": self.params.max_positions,
            },
            "state": self.state.to_string(),
            "last_updated": self.last_updated.map(|dt| dt.to_rfc3339()),
        }))
        .map_err(|e| crate::error::TradingError::StrategyError(
            format!("전략 직렬화 오류: {}", e)
        ))?;
        
        Ok(serialized)
    }
    
    async fn shutdown(&mut self) -> Result<()> {
        self.state = StrategyState::Stopped;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    fn create_test_candles() -> Vec<Candle> {
        let symbol = SymbolPair::new("BTC", "USDT");
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        
        // 시간별 종가 데이터 (빠른 MA가 느린 MA를 상향 돌파하는 패턴)
        let closes = vec![
            100.0, 101.0, 102.0, 103.0, 104.0, // 시작 데이터
            103.0, 102.0, 101.0, 100.0, 99.0,  // 하락 트렌드
            98.0, 97.0, 96.0, 95.0, 94.0,
            93.0, 92.0, 93.0, 94.0, 95.0,      // 반등 시작
            97.0, 99.0, 101.0, 103.0, 105.0,   // 상승 트렌드 (골든 크로스 예상)
            106.0, 107.0, 108.0, 109.0, 110.0,
        ];
        
        let mut candles = Vec::with_capacity(closes.len());
        
        for (i, close) in closes.iter().enumerate() {
            let time = base_time + chrono::Duration::hours(i as i64);
            candles.push(Candle {
                symbol: symbol.clone(),
                timestamp: time,
                open: *close - 1.0,
                high: *close + 1.0,
                low: *close - 2.0,
                close: *close,
                volume: 100.0 + (i as f64),
            });
        }
        
        candles
    }
    
    #[tokio::test]
    async fn test_moving_average_strategy() {
        let symbol = SymbolPair::new("BTC", "USDT");
        let mut strategy = MovingAverageCrossoverStrategy::new(
            "Test MA Crossover",
            symbol.clone(),
            5,  // 빠른 이동평균 기간
            10, // 느린 이동평균 기간
            0.1, // 포지션 사이즈
            Some(0.05), // 5% 스톱로스
            Some(0.15), // 15% 테이크프로핏
        );
        
        strategy.initialize().await.unwrap();
        
        let candles = create_test_candles();
        let signal = strategy.generate_signal(&candles).await.unwrap();
        
        // 골든 크로스 발생 확인
        assert!(signal.is_some());
        let signal = signal.unwrap();
        assert_eq!(signal.side, OrderSide::Buy);
        assert_eq!(signal.strength, SignalStrength::Strong);
        
        // 진입 결정
        let decision = strategy.decide_entry(&signal, None).await.unwrap();
        assert!(decision.is_some());
        let decision = decision.unwrap();
        assert_eq!(decision.action, crate::models::TradeAction::Enter);
        assert_eq!(decision.side, OrderSide::Buy);
    }
} 