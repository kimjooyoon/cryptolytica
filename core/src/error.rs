//! 오류 처리 모듈
//!
//! 이 모듈은 CryptoLytica 코어 라이브러리에서 사용되는 모든 오류 유형을 정의합니다.

use thiserror::Error;
use std::io;

/// 데이터 수집기 오류
#[derive(Debug, Error)]
pub enum CollectorError {
    /// 거래소가 지원되지 않음
    #[error("지원되지 않는 거래소: {0}")]
    UnsupportedExchange(String),
    
    /// API 인증 오류
    #[error("API 인증 오류: {0}")]
    Authentication(String),
    
    /// API 요청 오류
    #[error("API 요청 오류: {0}")]
    Request(String),
    
    /// API 응답 파싱 오류
    #[error("API 응답 파싱 오류: {0}")]
    ResponseParsing(String),
    
    /// 속도 제한(Rate Limit) 초과
    #[error("속도 제한 초과. 재시도 가능 시간: {0}")]
    RateLimited(String),
    
    /// 네트워크 오류
    #[error("네트워크 오류: {0}")]
    Network(String),
    
    /// 심볼 형식 오류
    #[error("잘못된 심볼 형식: {0}")]
    InvalidSymbol(String),
    
    /// 시간 간격 형식 오류
    #[error("잘못된 시간 간격 형식: {0}")]
    InvalidInterval(String),
    
    /// WebSocket 연결 오류
    #[error("WebSocket 연결 오류: {0}")]
    WebSocketConnection(String),
    
    /// HTTP 클라이언트 오류
    #[error("HTTP 클라이언트 오류: {0}")]
    HttpClient(#[from] reqwest::Error),
    
    /// 입출력 오류
    #[error("입출력 오류: {0}")]
    Io(#[from] io::Error),
    
    /// 일반 오류
    #[error("{0}")]
    General(String),
}

/// 데이터 처리기 오류
#[derive(Debug, Error)]
pub enum ProcessorError {
    /// 지원되지 않는 데이터 형식
    #[error("지원되지 않는 데이터 형식: {0}")]
    UnsupportedDataFormat(String),
    
    /// 데이터 변환 오류
    #[error("데이터 변환 오류: {0}")]
    DataTransformation(String),
    
    /// 파이프라인 구성 오류
    #[error("파이프라인 구성 오류: {0}")]
    PipelineConfiguration(String),
    
    /// 처리 오류
    #[error("데이터 처리 오류: {0}")]
    Processing(String),
    
    /// 입출력 오류
    #[error("입출력 오류: {0}")]
    Io(#[from] io::Error),
    
    /// 일반 오류
    #[error("{0}")]
    General(String),
}

/// 스토리지 오류
#[derive(Debug, Error)]
pub enum StorageError {
    /// 연결 오류
    #[error("데이터베이스 연결 오류: {0}")]
    Connection(String),
    
    /// 쿼리 오류
    #[error("쿼리 오류: {0}")]
    Query(String),
    
    /// 데이터 삽입 오류
    #[error("데이터 삽입 오류: {0}")]
    Insertion(String),
    
    /// 데이터 조회 오류
    #[error("데이터 조회 오류: {0}")]
    Retrieval(String),
    
    /// 스키마 오류
    #[error("스키마 오류: {0}")]
    Schema(String),
    
    /// 직렬화 오류
    #[error("직렬화 오류: {0}")]
    Serialization(String),
    
    /// 역직렬화 오류
    #[error("역직렬화 오류: {0}")]
    Deserialization(String),
    
    /// 입출력 오류
    #[error("입출력 오류: {0}")]
    Io(#[from] io::Error),
    
    /// 일반 오류
    #[error("{0}")]
    General(String),
}

/// CryptoLytica 코어 오류
#[derive(Debug, Error)]
pub enum CoreError {
    /// 수집기 오류
    #[error("수집기 오류: {0}")]
    Collector(#[from] CollectorError),
    
    /// 처리기 오류
    #[error("처리기 오류: {0}")]
    Processor(#[from] ProcessorError),
    
    /// 스토리지 오류
    #[error("스토리지 오류: {0}")]
    Storage(#[from] StorageError),
    
    /// 구성 오류
    #[error("구성 오류: {0}")]
    Configuration(String),
    
    /// 일반 오류
    #[error("{0}")]
    General(String),
}