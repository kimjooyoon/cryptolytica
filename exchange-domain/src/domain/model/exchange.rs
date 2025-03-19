use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// 거래소 식별자
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExchangeId(pub String);

impl ExchangeId {
    /// 새 거래소 ID 생성
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into().to_lowercase())
    }
    
    /// 거래소 ID 문자열 반환
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ExchangeId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ExchangeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 거래소 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExchangeType {
    /// 중앙화 거래소
    Centralized,
    /// 탈중앙화 거래소
    Decentralized,
    /// 하이브리드 거래소
    Hybrid,
}

/// 거래소 상태
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExchangeStatus {
    /// 활성화 상태
    Active,
    /// 비활성화 상태
    Inactive,
    /// 유지보수 중
    Maintenance,
    /// 일시적인 오류 상태
    Error,
}

/// 거래소 엔티티
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exchange {
    /// 고유 식별자 (UUID)
    pub id: Uuid,
    /// 거래소 코드 (binance, upbit 등)
    pub exchange_id: ExchangeId,
    /// 거래소 이름
    pub name: String,
    /// 거래소 타입
    pub exchange_type: ExchangeType,
    /// 거래소 상태
    pub status: ExchangeStatus,
    /// 거래소 기본 URL
    pub base_url: String,
    /// 거래소 웹소켓 URL
    pub websocket_url: Option<String>,
    /// 거래소가 지원하는 기능
    pub features: HashMap<String, bool>,
    /// 속도 제한 정보
    pub rate_limits: HashMap<String, u32>,
    /// 거래소 국가
    pub country: Option<String>,
    /// 생성 시간
    pub created_at: DateTime<Utc>,
    /// 마지막 업데이트 시간
    pub updated_at: DateTime<Utc>,
}

impl Exchange {
    /// 새 거래소 객체 생성
    pub fn new(
        exchange_id: ExchangeId, 
        name: impl Into<String>,
        exchange_type: ExchangeType,
        base_url: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            exchange_id,
            name: name.into(),
            exchange_type,
            status: ExchangeStatus::Active,
            base_url: base_url.into(),
            websocket_url: None,
            features: HashMap::new(),
            rate_limits: HashMap::new(),
            country: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// 거래소 상태 변경
    pub fn update_status(&mut self, status: ExchangeStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }
    
    /// 기능 지원 여부 설정
    pub fn set_feature(&mut self, feature: impl Into<String>, supported: bool) {
        self.features.insert(feature.into(), supported);
        self.updated_at = Utc::now();
    }
    
    /// 기능 지원 여부 확인
    pub fn supports_feature(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
    
    /// 웹소켓 URL 설정
    pub fn set_websocket_url(&mut self, url: impl Into<String>) {
        self.websocket_url = Some(url.into());
        self.updated_at = Utc::now();
    }
    
    /// 활성 상태 확인
    pub fn is_active(&self) -> bool {
        self.status == ExchangeStatus::Active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exchange_creation() {
        let exchange = Exchange::new(
            ExchangeId::new("binance"),
            "Binance",
            ExchangeType::Centralized,
            "https://api.binance.com"
        );
        
        assert_eq!(exchange.exchange_id.value(), "binance");
        assert_eq!(exchange.name, "Binance");
        assert_eq!(exchange.exchange_type, ExchangeType::Centralized);
        assert_eq!(exchange.status, ExchangeStatus::Active);
        assert_eq!(exchange.base_url, "https://api.binance.com");
        assert!(exchange.websocket_url.is_none());
        assert!(exchange.features.is_empty());
    }
    
    #[test]
    fn test_feature_support() {
        let mut exchange = Exchange::new(
            ExchangeId::new("upbit"),
            "Upbit",
            ExchangeType::Centralized,
            "https://api.upbit.com"
        );
        
        assert!(!exchange.supports_feature("websocket"));
        
        exchange.set_feature("websocket", true);
        exchange.set_feature("futures", false);
        
        assert!(exchange.supports_feature("websocket"));
        assert!(!exchange.supports_feature("futures"));
        assert!(!exchange.supports_feature("margin"));
    }
    
    #[test]
    fn test_status_update() {
        let mut exchange = Exchange::new(
            ExchangeId::new("bithumb"),
            "Bithumb",
            ExchangeType::Centralized,
            "https://api.bithumb.com"
        );
        
        assert!(exchange.is_active());
        
        exchange.update_status(ExchangeStatus::Maintenance);
        assert_eq!(exchange.status, ExchangeStatus::Maintenance);
        assert!(!exchange.is_active());
    }
} 