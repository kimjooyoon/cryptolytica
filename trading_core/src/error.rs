//! 트레이딩 관련 오류 타입 정의
//!
//! 이 모듈은 트레이딩 전략, 백테스팅, 실행 과정에서 발생할 수 있는 오류 유형을 정의합니다.

use thiserror::Error;
use cryptolytica_common_core::error::CoreError;
use cryptolytica_exchange_core::error::ExchangeError;
use cryptolytica_market_data_core::error::MarketDataError;

/// 트레이딩 관련 오류
#[derive(Error, Debug)]
pub enum TradingError {
    #[error("전략 오류: {0}")]
    StrategyError(String),

    #[error("백테스트 오류: {0}")]
    BacktestError(String),

    #[error("실행 오류: {0}")]
    ExecutionError(String),

    #[error("리스크 관리 오류: {0}")]
    RiskManagementError(String),

    #[error("포트폴리오 오류: {0}")]
    PortfolioError(String),

    #[error("지표 계산 오류: {0}")]
    MetricsError(String),

    #[error("순서 오류: {0}")]
    OrderError(String),

    #[error("유효하지 않은 매개변수: {0}")]
    InvalidParameterError(String),

    #[error("데이터 부족 오류: {0}")]
    InsufficientDataError(String),

    #[error("시장 조건 오류: {0}")]
    MarketConditionError(String),

    #[error("계산 오버플로우: {0}")]
    CalculationOverflowError(String),

    #[error("제약 조건 위반: {0}")]
    ConstraintViolationError(String),

    #[error("공통 코어 오류: {0}")]
    CommonError(#[from] CoreError),

    #[error("거래소 오류: {0}")]
    ExchangeError(#[from] ExchangeError),

    #[error("시장 데이터 오류: {0}")]
    MarketDataError(#[from] MarketDataError),
}

/// TradingError 에서 CoreError로 변환 구현
impl From<TradingError> for CoreError {
    fn from(err: TradingError) -> Self {
        match err {
            TradingError::StrategyError(msg) => 
                CoreError::Data(format!("트레이딩 전략 오류: {}", msg)),
                
            TradingError::BacktestError(msg) => 
                CoreError::Data(format!("트레이딩 백테스트 오류: {}", msg)),
                
            TradingError::ExecutionError(msg) => 
                CoreError::Data(format!("트레이딩 실행 오류: {}", msg)),
                
            TradingError::RiskManagementError(msg) => 
                CoreError::Data(format!("트레이딩 리스크 관리 오류: {}", msg)),
                
            TradingError::PortfolioError(msg) => 
                CoreError::Data(format!("트레이딩 포트폴리오 오류: {}", msg)),
                
            TradingError::MetricsError(msg) => 
                CoreError::Data(format!("트레이딩 지표 계산 오류: {}", msg)),
                
            TradingError::OrderError(msg) => 
                CoreError::Data(format!("트레이딩 주문 오류: {}", msg)),
                
            TradingError::InvalidParameterError(msg) => 
                CoreError::Data(format!("트레이딩 매개변수 오류: {}", msg)),
                
            TradingError::InsufficientDataError(msg) => 
                CoreError::Data(format!("트레이딩 데이터 부족 오류: {}", msg)),
                
            TradingError::MarketConditionError(msg) => 
                CoreError::Data(format!("트레이딩 시장 조건 오류: {}", msg)),
                
            TradingError::CalculationOverflowError(msg) => 
                CoreError::Data(format!("트레이딩 계산 오버플로우: {}", msg)),
                
            TradingError::ConstraintViolationError(msg) => 
                CoreError::Data(format!("트레이딩 제약 조건 위반: {}", msg)),
                
            TradingError::CommonError(err) => err,
            TradingError::ExchangeError(err) => err.into(),
            TradingError::MarketDataError(err) => err.into(),
        }
    }
}

/// 결과 타입 단축형
pub type Result<T> = std::result::Result<T, TradingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let trading_err = TradingError::StrategyError("테스트 오류".to_string());
        let core_err: CoreError = trading_err.into();
        
        match core_err {
            CoreError::Data(msg) => {
                assert!(msg.contains("테스트 오류"));
                assert!(msg.contains("트레이딩 전략 오류"));
            },
            _ => panic!("잘못된 오류 변환"),
        }
    }
} 