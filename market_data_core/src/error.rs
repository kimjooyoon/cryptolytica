//! 시장 데이터 관련 오류 타입 정의
//!
//! 이 모듈은 시장 데이터 수집, 저장 및 쿼리 과정에서 발생할 수 있는 오류 유형을 정의합니다.

use thiserror::Error;
use cryptolytica_common_core::error::CoreError;
use cryptolytica_exchange_core::error::ExchangeError;

/// 시장 데이터 관련 오류
#[derive(Error, Debug)]
pub enum MarketDataError {
    #[error("데이터 수집 오류: {0}")]
    CollectionError(String),

    #[error("데이터 정규화 오류: {0}")]
    NormalizationError(String),

    #[error("데이터 저장 오류: {0}")]
    StorageError(String),

    #[error("SQL 오류: {0}")]
    SqlError(String),

    #[error("쿼리 오류: {0}")]
    QueryError(String),

    #[error("시계열 오류: {0}")]
    TimeSeriesError(String),

    #[error("날짜 범위 오류: {0}")]
    DateRangeError(String),

    #[error("심볼 오류: {0}")]
    SymbolError(String),

    #[error("유효하지 않은 매개변수: {0}")]
    InvalidParameterError(String),

    #[error("데이터 변환 오류: {0}")]
    DataConversionError(String),

    #[error("거래소 오류: {0}")]
    ExchangeError(#[from] ExchangeError),

    #[error("공통 코어 오류: {0}")]
    CommonError(#[from] CoreError),
}

/// MarketDataError 에서 CoreError로 변환 구현
impl From<MarketDataError> for CoreError {
    fn from(err: MarketDataError) -> Self {
        match err {
            MarketDataError::CollectionError(msg) => 
                CoreError::Data(format!("시장 데이터 수집 오류: {}", msg)),
                
            MarketDataError::NormalizationError(msg) => 
                CoreError::Data(format!("시장 데이터 정규화 오류: {}", msg)),
                
            MarketDataError::StorageError(msg) => 
                CoreError::Data(format!("시장 데이터 저장 오류: {}", msg)),
                
            MarketDataError::SqlError(msg) => 
                CoreError::Data(format!("시장 데이터 SQL 오류: {}", msg)),
                
            MarketDataError::QueryError(msg) => 
                CoreError::Data(format!("시장 데이터 쿼리 오류: {}", msg)),
                
            MarketDataError::TimeSeriesError(msg) => 
                CoreError::Data(format!("시장 데이터 시계열 오류: {}", msg)),
                
            MarketDataError::DateRangeError(msg) => 
                CoreError::Data(format!("시장 데이터 날짜 범위 오류: {}", msg)),
                
            MarketDataError::SymbolError(msg) => 
                CoreError::Data(format!("시장 데이터 심볼 오류: {}", msg)),
                
            MarketDataError::InvalidParameterError(msg) => 
                CoreError::Data(format!("시장 데이터 유효하지 않은 매개변수: {}", msg)),
                
            MarketDataError::DataConversionError(msg) => 
                CoreError::Data(format!("시장 데이터 변환 오류: {}", msg)),
                
            MarketDataError::ExchangeError(err) => err.into(),
            MarketDataError::CommonError(err) => err,
        }
    }
}

/// MarketDataError에서 데이터베이스 특정 오류를 처리하는 유틸리티 함수
pub fn from_db_error(db_error: &str) -> MarketDataError {
    if db_error.contains("no such table") || db_error.contains("relation") {
        return MarketDataError::SqlError(format!("테이블이 존재하지 않음: {}", db_error));
    }
    
    if db_error.contains("constraint") {
        return MarketDataError::SqlError(format!("제약 조건 위반: {}", db_error));
    }
    
    if db_error.contains("duplicate") {
        return MarketDataError::SqlError(format!("중복 데이터: {}", db_error));
    }
    
    MarketDataError::SqlError(db_error.to_string())
}

/// 결과 타입 단축형
pub type Result<T> = std::result::Result<T, MarketDataError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let market_data_err = MarketDataError::CollectionError("테스트 오류".to_string());
        let core_err: CoreError = market_data_err.into();
        
        match core_err {
            CoreError::Data(msg) => {
                assert!(msg.contains("테스트 오류"));
                assert!(msg.contains("시장 데이터 수집 오류"));
            },
            _ => panic!("잘못된 오류 변환"),
        }
    }
    
    #[test]
    fn test_from_db_error() {
        let no_table_err = from_db_error("no such table: market_data");
        match no_table_err {
            MarketDataError::SqlError(msg) => {
                assert!(msg.contains("테이블이 존재하지 않음"));
            },
            _ => panic!("잘못된 DB 오류 처리"),
        }
    }
}