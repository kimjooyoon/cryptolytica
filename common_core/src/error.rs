//! 오류 처리 모듈
//!
//! 이 모듈은 CryptoLytica 프로젝트에서 사용되는 오류 타입과 처리 메커니즘을 정의합니다.

use thiserror::Error;
use std::fmt;

/// 공통 오류 타입
#[derive(Error, Debug)]
pub enum CoreError {
    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON 직렬화/역직렬화 오류: {0}")]
    Json(#[from] serde_json::Error),

    #[error("요청 오류: {0}")]
    Request(String),

    #[error("응답 오류 (코드: {code}): {message}")]
    Response {
        code: u16,
        message: String,
    },

    #[error("타임아웃 오류: {0}")]
    Timeout(String),

    #[error("인증 오류: {0}")]
    Authentication(String),

    #[error("권한 오류: {0}")]
    Authorization(String),

    #[error("데이터 오류: {0}")]
    Data(String),

    #[error("구성 오류: {0}")]
    Configuration(String),

    #[error("알 수 없는 오류: {0}")]
    Unknown(String),
}

/// 결과 타입 단축형
pub type Result<T> = std::result::Result<T, CoreError>;

/// 오류 코드 열거형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    Success = 0,
    GeneralError = 1000,
    NetworkError = 1100,
    ValidationError = 1200,
    AuthenticationError = 1300,
    AuthorizationError = 1400,
    DataError = 1500,
    ConfigurationError = 1600,
    TimeoutError = 1700,
    NotFoundError = 1800,
    DuplicateError = 1900,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, *self as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = CoreError::Request("테스트 오류".to_string());
        assert!(err.to_string().contains("테스트 오류"));
    }

    #[test]
    fn test_error_code_display() {
        assert_eq!(ErrorCode::Success.to_string(), "Success (0)");
    }
} 