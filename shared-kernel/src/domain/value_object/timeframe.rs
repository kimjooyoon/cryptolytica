use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// 타임프레임 파싱 오류
#[derive(Debug, Error)]
pub enum TimeframeError {
    #[error("잘못된 타임프레임 형식: {0}")]
    InvalidFormat(String),
    
    #[error("지원되지 않는 타임프레임: {0}")]
    UnsupportedTimeframe(String),
}

/// 타임프레임 (차트 기간) 값 객체
///
/// 금융 차트에서 사용되는 기간 단위
/// 예: 1분, 5분, 1시간, 1일 등
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Timeframe {
    /// 1분
    Minute1,
    /// 5분
    Minute5,
    /// 15분
    Minute15,
    /// 30분
    Minute30,
    /// 1시간
    Hour1,
    /// 4시간
    Hour4,
    /// 12시간
    Hour12,
    /// 1일
    Day1,
    /// 1주
    Week1,
    /// 1개월
    Month1,
}

impl Timeframe {
    /// 타임프레임의 기간(Duration)을 반환
    pub fn duration(&self) -> Duration {
        match self {
            Timeframe::Minute1 => Duration::from_secs(60),
            Timeframe::Minute5 => Duration::from_secs(5 * 60),
            Timeframe::Minute15 => Duration::from_secs(15 * 60),
            Timeframe::Minute30 => Duration::from_secs(30 * 60),
            Timeframe::Hour1 => Duration::from_secs(60 * 60),
            Timeframe::Hour4 => Duration::from_secs(4 * 60 * 60),
            Timeframe::Hour12 => Duration::from_secs(12 * 60 * 60),
            Timeframe::Day1 => Duration::from_secs(24 * 60 * 60),
            Timeframe::Week1 => Duration::from_secs(7 * 24 * 60 * 60),
            Timeframe::Month1 => Duration::from_secs(30 * 24 * 60 * 60), // 근사값
        }
    }
    
    /// 표준 코드 문자열로 변환 (예: "1m", "1h")
    pub fn to_code(&self) -> &'static str {
        match self {
            Timeframe::Minute1 => "1m",
            Timeframe::Minute5 => "5m",
            Timeframe::Minute15 => "15m",
            Timeframe::Minute30 => "30m",
            Timeframe::Hour1 => "1h",
            Timeframe::Hour4 => "4h",
            Timeframe::Hour12 => "12h",
            Timeframe::Day1 => "1d",
            Timeframe::Week1 => "1w",
            Timeframe::Month1 => "1M",
        }
    }
    
    /// 현재 시간을 기준으로 n개 이전의 캔들 시작시간 계산
    pub fn previous_candle_start(&self, n: usize, reference_time: Option<DateTime<Utc>>) -> DateTime<Utc> {
        let now = reference_time.unwrap_or_else(Utc::now);
        let duration_millis = self.duration().as_millis() as i64;
        let n_millis = (n as i64) * duration_millis;
        
        let timestamp_millis = now.timestamp_millis();
        let current_candle_start_millis = (timestamp_millis / duration_millis) * duration_millis;
        let previous_candle_start_millis = current_candle_start_millis - n_millis;
        
        DateTime::from_timestamp_millis(previous_candle_start_millis)
            .unwrap_or_else(|| now)
    }
    
    /// 모든 타임프레임 값 목록 반환
    pub fn all() -> Vec<Timeframe> {
        vec![
            Timeframe::Minute1,
            Timeframe::Minute5,
            Timeframe::Minute15,
            Timeframe::Minute30,
            Timeframe::Hour1,
            Timeframe::Hour4,
            Timeframe::Hour12,
            Timeframe::Day1,
            Timeframe::Week1,
            Timeframe::Month1,
        ]
    }
}

impl FromStr for Timeframe {
    type Err = TimeframeError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        
        match s.as_str() {
            "1m" => Ok(Timeframe::Minute1),
            "5m" => Ok(Timeframe::Minute5),
            "15m" => Ok(Timeframe::Minute15),
            "30m" => Ok(Timeframe::Minute30),
            "1h" => Ok(Timeframe::Hour1),
            "4h" => Ok(Timeframe::Hour4),
            "12h" => Ok(Timeframe::Hour12),
            "1d" => Ok(Timeframe::Day1),
            "1w" => Ok(Timeframe::Week1),
            "1M" | "1mo" => Ok(Timeframe::Month1),
            _ => {
                // 추가 파싱 시도: 숫자+단위 형식
                if let Some(last_char) = s.chars().last() {
                    let value = &s[0..s.len()-1];
                    if let Ok(num) = value.parse::<u32>() {
                        match last_char {
                            'm' => match num {
                                1 => Ok(Timeframe::Minute1),
                                5 => Ok(Timeframe::Minute5),
                                15 => Ok(Timeframe::Minute15),
                                30 => Ok(Timeframe::Minute30),
                                _ => Err(TimeframeError::UnsupportedTimeframe(s)),
                            },
                            'h' => match num {
                                1 => Ok(Timeframe::Hour1),
                                4 => Ok(Timeframe::Hour4),
                                12 => Ok(Timeframe::Hour12),
                                _ => Err(TimeframeError::UnsupportedTimeframe(s)),
                            },
                            'd' => match num {
                                1 => Ok(Timeframe::Day1),
                                _ => Err(TimeframeError::UnsupportedTimeframe(s)),
                            },
                            'w' => match num {
                                1 => Ok(Timeframe::Week1),
                                _ => Err(TimeframeError::UnsupportedTimeframe(s)),
                            },
                            'M' => match num {
                                1 => Ok(Timeframe::Month1),
                                _ => Err(TimeframeError::UnsupportedTimeframe(s)),
                            },
                            _ => Err(TimeframeError::InvalidFormat(s)),
                        }
                    } else {
                        Err(TimeframeError::InvalidFormat(s))
                    }
                } else {
                    Err(TimeframeError::InvalidFormat(s))
                }
            }
        }
    }
}

impl fmt::Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_code())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_timeframe_codes() {
        assert_eq!(Timeframe::Minute1.to_code(), "1m");
        assert_eq!(Timeframe::Hour1.to_code(), "1h");
        assert_eq!(Timeframe::Day1.to_code(), "1d");
    }
    
    #[test]
    fn test_timeframe_durations() {
        assert_eq!(Timeframe::Minute1.duration(), Duration::from_secs(60));
        assert_eq!(Timeframe::Hour1.duration(), Duration::from_secs(60 * 60));
        assert_eq!(Timeframe::Day1.duration(), Duration::from_secs(24 * 60 * 60));
    }
    
    #[test]
    fn test_timeframe_parsing() {
        assert_eq!(Timeframe::from_str("1m").unwrap(), Timeframe::Minute1);
        assert_eq!(Timeframe::from_str("5m").unwrap(), Timeframe::Minute5);
        assert_eq!(Timeframe::from_str("1h").unwrap(), Timeframe::Hour1);
        assert_eq!(Timeframe::from_str("1d").unwrap(), Timeframe::Day1);
        
        assert!(Timeframe::from_str("2h").is_err());
        assert!(Timeframe::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_timeframe_display() {
        assert_eq!(Timeframe::Minute5.to_string(), "5m");
        assert_eq!(Timeframe::Hour4.to_string(), "4h");
    }
} 