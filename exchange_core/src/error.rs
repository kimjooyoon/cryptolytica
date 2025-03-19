//! 거래소 연동 관련 오류 타입 정의
//!
//! 이 모듈은 거래소 API 통신 과정에서 발생할 수 있는 다양한 오류 유형을 정의합니다.

use thiserror::Error;
use cryptolytica_common_core::error::CoreError;

/// 거래소 관련 오류 정의
#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("API 요청 오류: {0}")]
    RequestError(String),

    #[error("API 응답 오류 (코드: {code}): {message}")]
    ResponseError {
        code: String,
        message: String,
    },

    #[error("인증 오류: {0}")]
    AuthenticationError(String),

    #[error("속도 제한 초과: {0}")]
    RateLimitExceeded(String),

    #[error("WebSocket 오류: {0}")]
    WebSocketError(String),

    #[error("거래소 응답 파싱 오류: {0}")]
    ParseError(String),

    #[error("타임아웃 오류: {0}")]
    TimeoutError(String),

    #[error("잘못된 요청 매개변수: {0}")]
    InvalidRequestParams(String),

    #[error("지원되지 않는 기능: {0}")]
    UnsupportedFeature(String),

    #[error("네트워크 오류: {0}")]
    NetworkError(String),

    #[error("내부 오류: {0}")]
    InternalError(String),

    #[error("공통 코어 오류: {0}")]
    CommonError(#[from] CoreError),
}

/// ExchangeError 에서 CoreError로 변환 구현
impl From<ExchangeError> for CoreError {
    fn from(err: ExchangeError) -> Self {
        match err {
            ExchangeError::RequestError(msg) => 
                CoreError::Request(format!("거래소 요청 오류: {}", msg)),
                
            ExchangeError::ResponseError { code, message } => 
                CoreError::Response { 
                    code: code.parse().unwrap_or(400), 
                    message: format!("거래소 응답 오류: {}", message) 
                },
                
            ExchangeError::AuthenticationError(msg) => 
                CoreError::Authentication(format!("거래소 인증 오류: {}", msg)),
                
            ExchangeError::RateLimitExceeded(msg) => 
                CoreError::Request(format!("거래소 속도 제한: {}", msg)),
                
            ExchangeError::WebSocketError(msg) => 
                CoreError::Request(format!("거래소 WebSocket 오류: {}", msg)),
                
            ExchangeError::ParseError(msg) => 
                CoreError::Data(format!("거래소 응답 파싱 오류: {}", msg)),
                
            ExchangeError::TimeoutError(msg) => 
                CoreError::Timeout(format!("거래소 타임아웃: {}", msg)),
                
            ExchangeError::InvalidRequestParams(msg) => 
                CoreError::Data(format!("거래소 잘못된 요청: {}", msg)),
                
            ExchangeError::UnsupportedFeature(msg) => 
                CoreError::Configuration(format!("지원되지 않는 거래소 기능: {}", msg)),
                
            ExchangeError::NetworkError(msg) => 
                CoreError::Request(format!("거래소 네트워크 오류: {}", msg)),
                
            ExchangeError::InternalError(msg) => 
                CoreError::Unknown(format!("거래소 내부 오류: {}", msg)),
                
            ExchangeError::CommonError(err) => err,
        }
    }
}

/// HTTP 오류로부터 ExchangeError 생성 유틸리티 함수
pub fn from_http_error(status: u16, body: &str) -> ExchangeError {
    if status == 429 {
        return ExchangeError::RateLimitExceeded(format!("HTTP {}: {}", status, body));
    }
    
    if status == 401 || status == 403 {
        return ExchangeError::AuthenticationError(format!("HTTP {}: {}", status, body));
    }
    
    if status >= 400 && status < 500 {
        return ExchangeError::RequestError(format!("HTTP {}: {}", status, body));
    }
    
    if status >= 500 {
        return ExchangeError::ResponseError { 
            code: status.to_string(), 
            message: body.to_string(),
        };
    }
    
    ExchangeError::InternalError(format!("예상치 못한 HTTP 오류 {}: {}", status, body))
}

/// 결과 타입 단축형
pub type Result<T> = std::result::Result<T, ExchangeError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_conversion() {
        let exchange_err = ExchangeError::RequestError("테스트 오류".to_string());
        let core_err: CoreError = exchange_err.into();
        
        match core_err {
            CoreError::Request(msg) => {
                assert!(msg.contains("테스트 오류"));
                assert!(msg.contains("거래소 요청 오류"));
            },
            _ => panic!("잘못된 오류 변환"),
        }
    }
    
    #[test]
    fn test_from_http_error() {
        let rate_limit_err = from_http_error(429, "Too Many Requests");
        match rate_limit_err {
            ExchangeError::RateLimitExceeded(msg) => {
                assert!(msg.contains("429"));
                assert!(msg.contains("Too Many Requests"));
            },
            _ => panic!("잘못된 HTTP 오류 처리"),
        }
    }
} 