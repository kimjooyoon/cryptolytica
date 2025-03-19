//! 공통 유틸리티 기능
//!
//! 이 모듈은 전체 프로젝트에서 사용되는 유틸리티 함수들을 제공합니다.

use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use std::ops::Add;
use crate::error::CoreError;
use crate::types::Result;

/// 타임스탬프(밀리초)를 DateTime<Utc>로 변환
pub fn ms_timestamp_to_datetime(ts: i64) -> DateTime<Utc> {
    let seconds = ts / 1000;
    let nanoseconds = (ts % 1000) * 1_000_000;
    
    let naive = NaiveDateTime::from_timestamp_opt(seconds, nanoseconds as u32)
        .unwrap_or_else(|| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        
    Utc.from_utc_datetime(&naive)
}

/// DateTime<Utc>를 타임스탬프(밀리초)로 변환
pub fn datetime_to_ms_timestamp(dt: DateTime<Utc>) -> i64 {
    dt.timestamp() * 1000 + dt.timestamp_subsec_millis() as i64
}

/// 문자열을 소수점(f64)으로 변환
pub fn parse_decimal_string(s: &str) -> Result<f64> {
    s.trim().parse::<f64>().map_err(|e| 
        CoreError::Data(format!("문자열 '{}' 파싱 실패: {}", s, e))
    )
}

/// 숫자 포맷팅 (소수점 제한)
pub fn format_decimal(value: f64, precision: usize) -> String {
    format!("{:.precision$}", value, precision = precision)
}

/// 백분율 계산 (변화율)
pub fn calculate_percentage_change(old_value: f64, new_value: f64) -> f64 {
    if old_value == 0.0 {
        return 0.0;
    }
    ((new_value - old_value) / old_value) * 100.0
}

/// 이동 평균 계산
pub fn calculate_simple_moving_average(values: &[f64], period: usize) -> Vec<f64> {
    if period == 0 || values.is_empty() || period > values.len() {
        return Vec::new();
    }
    
    let mut result = Vec::with_capacity(values.len() - period + 1);
    
    for i in 0..=values.len() - period {
        let sum: f64 = values[i..i+period].iter().sum();
        result.push(sum / period as f64);
    }
    
    result
}

/// UUID 생성
pub fn generate_uuid() -> String {
    format!("{}", uuid::Uuid::new_v4())
}

/// 시간 간격 문자열 파싱 (예: "1h", "30m", "1d")
pub fn parse_duration(duration_str: &str) -> Result<chrono::Duration> {
    let len = duration_str.len();
    if len < 2 {
        return Err(CoreError::Data(format!(
            "잘못된 기간 형식: '{}'", duration_str
        )));
    }
    
    let (value_str, unit) = duration_str.split_at(len - 1);
    let value = value_str.parse::<i64>().map_err(|_| {
        CoreError::Data(format!("기간 값 파싱 실패: '{}'", value_str))
    })?;
    
    match unit {
        "s" => Ok(chrono::Duration::seconds(value)),
        "m" => Ok(chrono::Duration::minutes(value)),
        "h" => Ok(chrono::Duration::hours(value)),
        "d" => Ok(chrono::Duration::days(value)),
        "w" => Ok(chrono::Duration::weeks(value)),
        _ => Err(CoreError::Data(format!(
            "알 수 없는 기간 단위: '{}'", unit
        ))),
    }
}

/// 현재 시간에 기간 추가
pub fn add_duration_to_now(duration_str: &str) -> Result<DateTime<Utc>> {
    let duration = parse_duration(duration_str)?;
    Ok(Utc::now().add(duration))
}

/// 문자열 마스킹 (비밀번호, API 키 등의 일부를 *로 변환)
pub fn mask_sensitive_data(data: &str, visible_chars: usize) -> String {
    if data.len() <= visible_chars {
        return data.to_string();
    }
    
    let visible = &data[0..visible_chars];
    let masked = "*".repeat(data.len() - visible_chars);
    format!("{}{}", visible, masked)
}

/// 지정된 시간 내에 함수가 완료되었는지 확인하는 타임아웃 래퍼
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    let timeout_duration = std::time::Duration::from_millis(timeout_ms);
    
    match tokio::time::timeout(timeout_duration, future).await {
        Ok(result) => result,
        Err(_) => Err(CoreError::Timeout(format!(
            "작업이 지정된 시간({} ms) 내에 완료되지 않았습니다", timeout_ms
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_timestamp_conversion() {
        let now = Utc::now();
        let ts = datetime_to_ms_timestamp(now);
        let converted_back = ms_timestamp_to_datetime(ts);
        
        // 밀리초 단위까지만 비교 (변환 과정에서 나노초 정보 손실)
        assert_eq!(
            now.timestamp_millis(),
            converted_back.timestamp_millis()
        );
    }
    
    #[test]
    fn test_parse_decimal_string() {
        assert_eq!(parse_decimal_string("123.45").unwrap(), 123.45);
        assert_eq!(parse_decimal_string("0").unwrap(), 0.0);
        assert!(parse_decimal_string("not a number").is_err());
    }
    
    #[test]
    fn test_format_decimal() {
        assert_eq!(format_decimal(123.456789, 2), "123.46");
        assert_eq!(format_decimal(0.1, 5), "0.10000");
    }
    
    #[test]
    fn test_calculate_percentage_change() {
        assert_eq!(calculate_percentage_change(100.0, 110.0), 10.0);
        assert_eq!(calculate_percentage_change(100.0, 90.0), -10.0);
        assert_eq!(calculate_percentage_change(0.0, 10.0), 0.0);
    }
    
    #[test]
    fn test_calculate_simple_moving_average() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let sma_3 = calculate_simple_moving_average(&values, 3);
        assert_eq!(sma_3, vec![2.0, 3.0, 4.0]);
        
        let sma_2 = calculate_simple_moving_average(&values, 2);
        assert_eq!(sma_2, vec![1.5, 2.5, 3.5, 4.5]);
        
        let empty_sma = calculate_simple_moving_average(&values, 6);
        assert!(empty_sma.is_empty());
    }
    
    #[test]
    fn test_parse_duration() {
        assert_eq!(parse_duration("1s").unwrap(), Duration::seconds(1));
        assert_eq!(parse_duration("5m").unwrap(), Duration::minutes(5));
        assert_eq!(parse_duration("2h").unwrap(), Duration::hours(2));
        assert_eq!(parse_duration("1d").unwrap(), Duration::days(1));
        assert_eq!(parse_duration("2w").unwrap(), Duration::weeks(2));
        
        assert!(parse_duration("invalid").is_err());
        assert!(parse_duration("5x").is_err());
    }
    
    #[test]
    fn test_mask_sensitive_data() {
        assert_eq!(mask_sensitive_data("password123", 3), "pas*********");
        assert_eq!(mask_sensitive_data("api_key", 2), "ap*****");
        assert_eq!(mask_sensitive_data("abc", 3), "abc");
        assert_eq!(mask_sensitive_data("ab", 3), "ab");
    }
} 