use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

/// 암호화폐 거래 쌍(Symbol Pair) 값 객체
///
/// 암호화폐 거래 쌍은 기본 자산(base)과 견적 자산(quote)으로 구성됩니다.
/// 예: BTC/USDT, ETH/USD 등
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPair {
    /// 기본 자산 (예: 'BTC', 'ETH')
    base: String,
    /// 견적 자산 (예: 'USDT', 'USD')
    quote: String,
}

impl SymbolPair {
    /// 새로운 거래 쌍 생성
    pub fn new(base: impl Into<String>, quote: impl Into<String>) -> Self {
        Self {
            base: base.into().to_uppercase(),
            quote: quote.into().to_uppercase(),
        }
    }

    /// 기본 자산 조회
    pub fn base(&self) -> &str {
        &self.base
    }

    /// 견적 자산 조회
    pub fn quote(&self) -> &str {
        &self.quote
    }

    /// 표준 형식으로 변환 (예: "BTC/USDT")
    pub fn to_standard_notation(&self) -> String {
        format!("{}/{}", self.base, self.quote)
    }

    /// 거래소별 형식으로 변환 (예: "BTCUSDT")
    pub fn to_exchange_format(&self, delimiter: Option<&str>) -> String {
        match delimiter {
            Some(delim) => format!("{}{}{}", self.base, delim, self.quote),
            None => format!("{}{}", self.base, self.quote),
        }
    }

    /// 문자열에서 심볼 쌍 파싱
    /// 
    /// `BTC/USDT`, `BTC-USDT`, `BTCUSDT` 등의 형식 지원
    pub fn from_str(input: &str) -> Option<Self> {
        let input = input.trim().to_uppercase();
        
        // 형식에 따라 분리
        let (base, quote) = if input.contains('/') {
            let parts: Vec<&str> = input.split('/').collect();
            (parts.get(0)?.trim(), parts.get(1)?.trim())
        } else if input.contains('-') {
            let parts: Vec<&str> = input.split('-').collect();
            (parts.get(0)?.trim(), parts.get(1)?.trim())
        } else if input.contains('_') {
            let parts: Vec<&str> = input.split('_').collect();
            (parts.get(0)?.trim(), parts.get(1)?.trim())
        } else {
            // 구분자가 없을 경우 일반적으로 마지막 3-4 글자가 quote
            if input.len() <= 4 {
                return None;
            }
            
            let common_quotes = ["USDT", "USD", "BTC", "ETH", "EUR", "JPY"];
            
            for quote in common_quotes.iter() {
                if input.ends_with(quote) {
                    let quote_len = quote.len();
                    let base = &input[..input.len() - quote_len];
                    if !base.is_empty() {
                        return Some(SymbolPair::new(base, *quote));
                    }
                }
            }
            
            // 기본값: 마지막 4글자를 quote로 가정
            let split_at = input.len().saturating_sub(4);
            (&input[..split_at], &input[split_at..])
        };
        
        if base.is_empty() || quote.is_empty() {
            return None;
        }
        
        Some(SymbolPair::new(base, quote))
    }
}

impl PartialEq for SymbolPair {
    fn eq(&self, other: &Self) -> bool {
        self.base == other.base && self.quote == other.quote
    }
}

impl Eq for SymbolPair {}

impl Hash for SymbolPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
        self.quote.hash(state);
    }
}

impl fmt::Display for SymbolPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_pair_creation() {
        let pair = SymbolPair::new("btc", "usdt");
        assert_eq!(pair.base(), "BTC");
        assert_eq!(pair.quote(), "USDT");
        assert_eq!(pair.to_standard_notation(), "BTC/USDT");
    }
    
    #[test]
    fn test_symbol_pair_equality() {
        let pair1 = SymbolPair::new("BTC", "USDT");
        let pair2 = SymbolPair::new("btc", "usdt");
        let pair3 = SymbolPair::new("ETH", "USDT");
        
        assert_eq!(pair1, pair2);
        assert_ne!(pair1, pair3);
    }
    
    #[test]
    fn test_symbol_pair_formats() {
        let pair = SymbolPair::new("ETH", "BTC");
        
        assert_eq!(pair.to_standard_notation(), "ETH/BTC");
        assert_eq!(pair.to_exchange_format(None), "ETHBTC");
        assert_eq!(pair.to_exchange_format(Some("-")), "ETH-BTC");
    }
    
    #[test]
    fn test_from_str_parsing() {
        let test_cases = vec![
            ("BTC/USDT", Some(SymbolPair::new("BTC", "USDT"))),
            ("ETH-BTC", Some(SymbolPair::new("ETH", "BTC"))),
            ("XRP_USD", Some(SymbolPair::new("XRP", "USD"))),
            ("ADAUSDT", Some(SymbolPair::new("ADA", "USDT"))),
            ("BTCETH", Some(SymbolPair::new("BTC", "ETH"))),
            ("", None),
            ("BTC", None),
        ];
        
        for (input, expected) in test_cases {
            let result = SymbolPair::from_str(input);
            assert_eq!(result.as_ref().map(|p| p.base()), expected.as_ref().map(|p| p.base()));
            assert_eq!(result.as_ref().map(|p| p.quote()), expected.as_ref().map(|p| p.quote()));
        }
    }
} 