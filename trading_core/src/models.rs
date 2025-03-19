//! 트레이딩 관련 데이터 모델 정의
//!
//! 이 모듈은 트레이딩 전략, 신호, 포지션 등의 데이터 구조체를 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::fmt;

use cryptolytica_common_core::types::SymbolPair;
use cryptolytica_exchange_core::models::OrderSide;

/// 전략 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyType {
    /// 추세 추종 전략
    Trend,
    /// 평균 회귀 전략
    MeanReversion,
    /// 돌파 전략
    Breakout,
    /// 패턴 인식 전략
    Pattern,
    /// 통계적 차익거래 전략
    StatisticalArbitrage,
    /// 기본 전략
    Momentum,
    /// 기타 전략
    Other,
}

impl fmt::Display for StrategyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrategyType::Trend => write!(f, "trend"),
            StrategyType::MeanReversion => write!(f, "mean_reversion"),
            StrategyType::Breakout => write!(f, "breakout"),
            StrategyType::Pattern => write!(f, "pattern"),
            StrategyType::StatisticalArbitrage => write!(f, "statistical_arbitrage"),
            StrategyType::Momentum => write!(f, "momentum"),
            StrategyType::Other => write!(f, "other"),
        }
    }
}

/// 전략 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrategyState {
    /// 초기화됨
    Initialized,
    /// 실행 중
    Running,
    /// 일시 중지
    Paused,
    /// 중지됨
    Stopped,
    /// 오류
    Error,
}

impl fmt::Display for StrategyState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrategyState::Initialized => write!(f, "initialized"),
            StrategyState::Running => write!(f, "running"),
            StrategyState::Paused => write!(f, "paused"),
            StrategyState::Stopped => write!(f, "stopped"),
            StrategyState::Error => write!(f, "error"),
        }
    }
}

/// 신호 강도
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SignalStrength {
    /// 매우 약함
    VeryWeak = 1,
    /// 약함
    Weak = 2,
    /// 중간
    Medium = 3,
    /// 강함
    Strong = 4,
    /// 매우 강함
    VeryStrong = 5,
}

impl fmt::Display for SignalStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignalStrength::VeryWeak => write!(f, "very_weak"),
            SignalStrength::Weak => write!(f, "weak"),
            SignalStrength::Medium => write!(f, "medium"),
            SignalStrength::Strong => write!(f, "strong"),
            SignalStrength::VeryStrong => write!(f, "very_strong"),
        }
    }
}

/// 트레이딩 신호
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    /// 신호 고유 ID
    pub id: Uuid,
    /// 전략 ID
    pub strategy_id: Uuid,
    /// 거래 쌍
    pub symbol: SymbolPair,
    /// 신호 생성 시간
    pub timestamp: DateTime<Utc>,
    /// 매수/매도 방향
    pub side: OrderSide,
    /// 신호 강도
    pub strength: SignalStrength,
    /// 추천 가격 (옵션)
    pub price: Option<f64>,
    /// 신호 만료 시간 (옵션)
    pub expiration: Option<DateTime<Utc>>,
    /// 추가 메타데이터
    pub metadata: serde_json::Value,
}

/// 트레이딩 액션
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TradeAction {
    /// 포지션 진입
    Enter,
    /// 포지션 종료
    Exit,
    /// 포지션 사이즈 증가
    Increase,
    /// 포지션 사이즈 감소
    Decrease,
    /// 손절가 조정
    AdjustStopLoss,
    /// 이익실현가 조정
    AdjustTakeProfit,
}

impl fmt::Display for TradeAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TradeAction::Enter => write!(f, "enter"),
            TradeAction::Exit => write!(f, "exit"),
            TradeAction::Increase => write!(f, "increase"),
            TradeAction::Decrease => write!(f, "decrease"),
            TradeAction::AdjustStopLoss => write!(f, "adjust_stop_loss"),
            TradeAction::AdjustTakeProfit => write!(f, "adjust_take_profit"),
        }
    }
}

/// 트레이딩 결정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDecision {
    /// 결정 고유 ID
    pub id: Uuid,
    /// 전략 ID
    pub strategy_id: Uuid,
    /// 관련 신호 ID
    pub signal_id: Uuid,
    /// 결정 시간
    pub timestamp: DateTime<Utc>,
    /// 거래 쌍
    pub symbol: SymbolPair,
    /// 매수/매도 방향
    pub side: OrderSide,
    /// 액션 유형
    pub action: TradeAction,
    /// 예상 가격
    pub price: Option<f64>,
    /// 수량
    pub quantity: Option<f64>,
    /// 결정 이유
    pub reason: String,
    /// 손절 수준 (비율)
    pub stop_loss: Option<f64>,
    /// 이익실현 수준 (비율)
    pub take_profit: Option<f64>,
    /// 추가 메타데이터
    pub metadata: serde_json::Value,
}

/// 포지션 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionStatus {
    /// 시장 진입 대기 중
    Pending,
    /// 활성화 상태
    Active,
    /// 종료됨
    Closed,
}

impl fmt::Display for PositionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionStatus::Pending => write!(f, "pending"),
            PositionStatus::Active => write!(f, "active"),
            PositionStatus::Closed => write!(f, "closed"),
        }
    }
}

/// 포지션 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// 포지션 고유 ID
    pub id: Uuid,
    /// 전략 ID
    pub strategy_id: Uuid,
    /// 관련 신호 ID
    pub signal_id: Uuid,
    /// 거래 쌍
    pub symbol: SymbolPair,
    /// 매수/매도 방향
    pub side: OrderSide,
    /// 포지션 상태
    pub status: PositionStatus,
    /// 진입 가격
    pub entry_price: f64,
    /// 포지션 수량
    pub quantity: f64,
    /// 현재 시장 가격
    pub current_price: Option<f64>,
    /// 손절가
    pub stop_loss: Option<f64>,
    /// 이익실현가
    pub take_profit: Option<f64>,
    /// 진입 시간
    pub entry_time: Option<DateTime<Utc>>,
    /// 종료 시간
    pub exit_time: Option<DateTime<Utc>>,
    /// 종료 가격
    pub exit_price: Option<f64>,
    /// 미실현 손익 (달러)
    pub unrealized_pnl: Option<f64>,
    /// 실현 손익 (달러)
    pub realized_pnl: Option<f64>,
    /// 포지션 비용 (수수료 등)
    pub costs: f64,
    /// 포지션 수익률 (%)
    pub profit_percentage: Option<f64>,
    /// 추가 메타데이터
    pub metadata: serde_json::Value,
}

impl Position {
    /// 포지션의 현재 손익 계산
    pub fn calculate_pnl(&self) -> Option<f64> {
        let current_price = self.current_price?;
        
        match self.side {
            OrderSide::Buy => Some((current_price - self.entry_price) * self.quantity - self.costs),
            OrderSide::Sell => Some((self.entry_price - current_price) * self.quantity - self.costs),
        }
    }
    
    /// 포지션 수익률 계산
    pub fn calculate_profit_percentage(&self) -> Option<f64> {
        let pnl = self.calculate_pnl()?;
        let investment = self.entry_price * self.quantity;
        
        if investment == 0.0 {
            return None;
        }
        
        Some((pnl / investment) * 100.0)
    }
    
    /// 포지션 업데이트
    pub fn update_with_price(&mut self, current_price: f64) -> Option<f64> {
        self.current_price = Some(current_price);
        let pnl = self.calculate_pnl()?;
        self.unrealized_pnl = Some(pnl);
        self.profit_percentage = self.calculate_profit_percentage();
        
        // 손절 또는 이익실현 확인
        if let Some(stop_loss) = self.stop_loss {
            match self.side {
                OrderSide::Buy if current_price <= stop_loss => return Some(pnl),
                OrderSide::Sell if current_price >= stop_loss => return Some(pnl),
                _ => {}
            }
        }
        
        if let Some(take_profit) = self.take_profit {
            match self.side {
                OrderSide::Buy if current_price >= take_profit => return Some(pnl),
                OrderSide::Sell if current_price <= take_profit => return Some(pnl),
                _ => {}
            }
        }
        
        None
    }
}

/// 백테스트 결과 지표
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestMetrics {
    /// 시작 자본
    pub initial_capital: f64,
    /// 최종 자본
    pub final_capital: f64,
    /// 순 수익
    pub net_profit: f64,
    /// 수익률 (%)
    pub profit_percentage: f64,
    /// 연간 수익률 (%)
    pub annual_return: f64,
    /// 승률 (%)
    pub win_rate: f64,
    /// 총 거래 횟수
    pub total_trades: u32,
    /// 승리 거래 횟수
    pub winning_trades: u32,
    /// 패배 거래 횟수
    pub losing_trades: u32,
    /// 손익비
    pub profit_factor: f64,
    /// 최대 손실률 (%)
    pub max_drawdown: f64,
    /// 샤프 비율
    pub sharpe_ratio: f64,
    /// 소르티노 비율
    pub sortino_ratio: f64,
    /// 수익 거래 평균 (%)
    pub avg_profit_per_winning_trade: f64,
    /// 손실 거래 평균 (%)
    pub avg_loss_per_losing_trade: f64,
    /// 평균 보유 시간 (시간)
    pub avg_hold_time: f64,
    /// 추가 메타데이터
    pub metadata: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_signal_strength_ordering() {
        assert!(SignalStrength::Strong > SignalStrength::Medium);
        assert!(SignalStrength::VeryStrong > SignalStrength::Strong);
        assert!(SignalStrength::Weak < SignalStrength::Medium);
        assert!(SignalStrength::VeryWeak < SignalStrength::Weak);
    }
    
    #[test]
    fn test_position_pnl() {
        let mut long_position = Position {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            signal_id: Uuid::new_v4(),
            symbol: SymbolPair::new("BTC", "USDT"),
            side: OrderSide::Buy,
            status: PositionStatus::Active,
            entry_price: 40000.0,
            quantity: 0.1,
            current_price: Some(42000.0),
            stop_loss: Some(39000.0),
            take_profit: Some(45000.0),
            entry_time: Some(Utc::now()),
            exit_time: None,
            exit_price: None,
            unrealized_pnl: None,
            realized_pnl: None,
            costs: 10.0,
            profit_percentage: None,
            metadata: serde_json::json!({}),
        };
        
        // 계산
        let pnl = long_position.calculate_pnl().unwrap();
        assert_eq!(pnl, (42000.0 - 40000.0) * 0.1 - 10.0);
        
        // 반대 방향 테스트
        let mut short_position = Position {
            id: Uuid::new_v4(),
            strategy_id: Uuid::new_v4(),
            signal_id: Uuid::new_v4(),
            symbol: SymbolPair::new("BTC", "USDT"),
            side: OrderSide::Sell,
            status: PositionStatus::Active,
            entry_price: 40000.0,
            quantity: 0.1,
            current_price: Some(38000.0),
            stop_loss: Some(41000.0),
            take_profit: Some(35000.0),
            entry_time: Some(Utc::now()),
            exit_time: None,
            exit_price: None,
            unrealized_pnl: None,
            realized_pnl: None,
            costs: 10.0,
            profit_percentage: None,
            metadata: serde_json::json!({}),
        };
        
        // 계산
        let pnl = short_position.calculate_pnl().unwrap();
        assert_eq!(pnl, (40000.0 - 38000.0) * 0.1 - 10.0);
        
        // 업데이트 테스트
        let trigger_result = long_position.update_with_price(45000.0);
        assert!(trigger_result.is_some()); // 이익실현 발동
        assert_eq!(long_position.profit_percentage.unwrap(), 
            ((45000.0 - 40000.0) * 0.1 - 10.0) / (40000.0 * 0.1) * 100.0);
    }
} 