//! CryptoLytica Core - 암호화폐 데이터 수집 및 처리를 위한 핵심 라이브러리
//!
//! 이 라이브러리는 다양한 암호화폐 거래소에서 데이터를 수집하고 처리하는 고성능 도구를 제공합니다.
//! 실시간 시장 데이터, 역사적 데이터, 주문 정보 등을 효율적으로 처리합니다.

pub mod collectors;
pub mod models;
pub mod processors;
pub mod storage;
pub mod error;
pub mod utils;

/// 라이브러리 버전 정보
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// CryptoLytica 코어 설정
#[derive(Debug, Clone)]
pub struct Config {
    /// 로깅 레벨
    pub log_level: LogLevel,
    /// 스토리지 설정
    pub storage: StorageConfig,
    /// 요청 시간 초과(밀리초)
    pub timeout_ms: u64,
    /// 최대 동시 요청 수
    pub max_concurrent_requests: usize,
    /// 재시도 횟수
    pub retry_count: u8,
}

/// 로깅 레벨
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 스토리지 설정
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// 스토리지 유형
    pub storage_type: StorageType,
    /// 연결 문자열
    pub connection_string: String,
    /// 최대 연결 수
    pub max_connections: u32,
}

/// 스토리지 유형
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    Memory,
    Disk,
    TimescaleDB,
    ClickHouse,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: LogLevel::Info,
            storage: StorageConfig {
                storage_type: StorageType::Memory,
                connection_string: String::new(),
                max_connections: 10,
            },
            timeout_ms: 30000, // 30 seconds
            max_concurrent_requests: 100,
            retry_count: 3,
        }
    }
}

/// CryptoLytica Core의 진입점
pub struct CryptoLytica {
    config: Config,
}

impl CryptoLytica {
    /// 새로운 CryptoLytica 인스턴스 생성
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    /// 기본 설정으로 새로운 인스턴스 생성
    pub fn default() -> Self {
        Self {
            config: Config::default(),
        }
    }
    
    /// 현재 버전 정보 반환
    pub fn version(&self) -> &'static str {
        VERSION
    }
    
    /// 설정 정보 반환
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let crypto = CryptoLytica::default();
        assert_eq!(crypto.config().log_level, LogLevel::Info);
        assert_eq!(crypto.config().retry_count, 3);
    }
    
    #[test]
    fn test_version() {
        let crypto = CryptoLytica::default();
        assert!(!crypto.version().is_empty());
    }
}