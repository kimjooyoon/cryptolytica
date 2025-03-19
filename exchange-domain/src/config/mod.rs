//! 거래소 도메인 설정 모듈
//!
//! 이 모듈은 거래소 도메인에 필요한 설정을 관리합니다.
//! 12-Factor App 원칙에 따라 환경 변수를 통해 설정을 주입받습니다.

use config::{Config, ConfigError, File, Environment};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// 거래소 도메인 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeDomainConfig {
    /// 기본 HTTP 설정
    pub http: HttpConfig,
    /// 거래소별 설정
    pub exchanges: ExchangesConfig,
    /// 캐시 설정
    pub cache: CacheConfig,
    /// 속도 제한 설정
    pub rate_limit: RateLimitConfig,
}

/// HTTP 클라이언트 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    /// 요청 타임아웃 (밀리초)
    pub timeout_ms: u64,
    /// 연결 타임아웃 (밀리초)
    pub connect_timeout_ms: u64,
    /// 최대 연결 수
    pub max_connections: usize,
    /// User-Agent 헤더
    pub user_agent: String,
    /// 재시도 횟수
    pub retry_count: u32,
    /// 재시도 지연 시간 (밀리초)
    pub retry_delay_ms: u64,
}

/// 거래소별 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangesConfig {
    /// Binance 거래소 설정
    pub binance: Option<ExchangeConfig>,
    /// Upbit 거래소 설정
    pub upbit: Option<ExchangeConfig>,
    /// Bithumb 거래소 설정
    pub bithumb: Option<ExchangeConfig>,
    /// Coinbase 거래소 설정
    pub coinbase: Option<ExchangeConfig>,
    /// Bybit 거래소 설정
    pub bybit: Option<ExchangeConfig>,
    /// 기타 거래소 설정 (키-값 맵)
    #[serde(default)]
    pub others: std::collections::HashMap<String, ExchangeConfig>,
}

/// 개별 거래소 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeConfig {
    /// 거래소 기본 URL
    pub base_url: String,
    /// 웹소켓 URL
    pub websocket_url: Option<String>,
    /// API 키 (환경 변수로 오버라이드 가능)
    pub api_key: Option<String>,
    /// API 시크릿 (환경 변수로 오버라이드 가능)
    pub api_secret: Option<String>,
    /// 추가 인증 정보 (예: 패스프레이즈)
    pub extra_auth: Option<std::collections::HashMap<String, String>>,
    /// 거래소별 속도 제한 설정
    pub rate_limits: Option<std::collections::HashMap<String, u32>>,
    /// 활성화 여부
    pub enabled: bool,
}

/// 캐시 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 캐시 활성화 여부
    pub enabled: bool,
    /// 기본 TTL (초)
    pub default_ttl_seconds: u64,
    /// 최대 항목 수
    pub max_capacity: usize,
}

/// 속도 제한 설정
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// 속도 제한 활성화 여부
    pub enabled: bool,
    /// 글로벌 초당 최대 요청 수
    pub global_requests_per_second: u32,
    /// 버스트 모드 허용 여부
    pub allow_burst: bool,
}

/// 전역 설정 싱글톤
static CONFIG: OnceLock<ExchangeDomainConfig> = OnceLock::new();

/// 설정 로드 함수
pub fn load_config() -> Result<(), ConfigError> {
    // 환경 변수 로드
    let _ = dotenv();
    
    // 설정 빌더 생성
    let config_builder = Config::builder()
        // 기본 설정 파일
        .add_source(File::with_name("config/default").required(false))
        // 환경별 설정 파일
        .add_source(
            File::with_name(&format!("config/{}", 
                std::env::var("APP_ENV").unwrap_or_else(|_| "development".into())
            )).required(false)
        )
        // 로컬 오버라이드 설정 파일
        .add_source(File::with_name("config/local").required(false))
        // 환경 변수 (접두사: EXCHANGE_)
        .add_source(Environment::with_prefix("EXCHANGE").separator("__"));
    
    // 설정 빌드
    let config = config_builder.build()?;
    
    // 설정 역직렬화
    let exchange_config: ExchangeDomainConfig = config.try_deserialize()?;
    
    // 전역 싱글톤에 설정 저장
    let _ = CONFIG.set(exchange_config);
    
    Ok(())
}

/// 설정 가져오기
pub fn get_config() -> &'static ExchangeDomainConfig {
    CONFIG.get().expect("설정이 초기화되지 않았습니다. load_config()를 먼저 호출하세요.")
}

/// 테스트용 기본 설정 생성
#[cfg(test)]
pub fn create_test_config() -> ExchangeDomainConfig {
    ExchangeDomainConfig {
        http: HttpConfig {
            timeout_ms: 30000,
            connect_timeout_ms: 5000,
            max_connections: 100,
            user_agent: "CryptoLytica/0.1.0".to_string(),
            retry_count: 3,
            retry_delay_ms: 1000,
        },
        exchanges: ExchangesConfig {
            binance: Some(ExchangeConfig {
                base_url: "https://api.binance.com".to_string(),
                websocket_url: Some("wss://stream.binance.com:9443/ws".to_string()),
                api_key: None,
                api_secret: None,
                extra_auth: None,
                rate_limits: None,
                enabled: true,
            }),
            upbit: None,
            bithumb: None,
            coinbase: None,
            bybit: None,
            others: std::collections::HashMap::new(),
        },
        cache: CacheConfig {
            enabled: true,
            default_ttl_seconds: 60,
            max_capacity: 10000,
        },
        rate_limit: RateLimitConfig {
            enabled: true,
            global_requests_per_second: 50,
            allow_burst: true,
        },
    }
} 