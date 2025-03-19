//! 오류 처리 모듈
//!
//! 이 모듈은 CryptoLytica 프로젝트에서 사용되는 오류 타입과 처리 메커니즘을 정의합니다.

use thiserror::Error;
use std::fmt;

/// 핵심 오류 타입
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

    #[error("유효성 검사 오류: {0}")]
    Validation(String),

    #[error("도메인 오류: {0}")]
    Domain(String),

    #[error("검색 요소를 찾을 수 없음: {0}")]
    NotFound(String),

    #[error("알 수 없는 오류: {0}")]
    Unknown(String),
}

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
    DomainError = 2000,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, *self as i32)
    }
}

/// HTTP 오류에서 CoreError 생성
pub fn from_http_error(status: u16, body: &str) -> CoreError {
    match status {
        401 | 403 => CoreError::Authentication(format!("HTTP {}: {}", status, body)),
        404 => CoreError::NotFound(format!("HTTP {}: {}", status, body)),
        429 => CoreError::Timeout(format!("속도 제한 초과: HTTP {}: {}", status, body)),
        400..=499 => CoreError::Request(format!("HTTP {}: {}", status, body)),
        500..=599 => CoreError::Response {
            code: status,
            message: body.to_string(),
        },
        _ => CoreError::Unknown(format!("예상치 못한 HTTP 상태 코드 {}: {}", status, body)),
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

    #[test]
    fn test_from_http_error() {
        let auth_err = from_http_error(401, "Unauthorized");
        match auth_err {
            CoreError::Authentication(msg) => {
                assert!(msg.contains("401"));
                assert!(msg.contains("Unauthorized"));
            },
            _ => panic!("잘못된 에러 타입"),
        }

        let not_found_err = from_http_error(404, "Not Found");
        match not_found_err {
            CoreError::NotFound(msg) => {
                assert!(msg.contains("404"));
                assert!(msg.contains("Not Found"));
            },
            _ => panic!("잘못된 에러 타입"),
        }
    }
} 