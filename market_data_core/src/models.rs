//! 시장 데이터 모델 정의
//!
//! 이 모듈은 시장 데이터를 표현하는 데이터 구조체들을 정의합니다.

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use polars::prelude::*;
use uuid::Uuid;

use cryptolytica_common_core::types::{SymbolPair, ExchangeId, Timeframe, Candle, Price};

/// 시장 데이터 컬렉션 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataCollection {
    /// 컬렉션 고유 ID
    pub id: Uuid,
    /// 컬렉션 이름
    pub name: String,
    /// 컬렉션 설명
    pub description: Option<String>,
    /// 데이터 시작 시간
    pub start_time: DateTime<Utc>,
    /// 데이터 종료 시간
    pub end_time: DateTime<Utc>,
    /// 마지막 업데이트 시간
    pub last_updated: DateTime<Utc>,
    /// 데이터 포인트 수
    pub data_points: u64,
    /// 컬렉션에 포함된 심볼들
    pub symbols: Vec<SymbolPair>,
    /// 컬렉션에 포함된 거래소들
    pub exchanges: Vec<ExchangeId>,
    /// 데이터 타입 (캔들, 틱, 등)
    pub data_type: MarketDataType,
    /// 타임프레임 (캔들 데이터일 경우)
    pub timeframe: Option<Timeframe>,
}

/// 시장 데이터 타입
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MarketDataType {
    #[serde(rename = "candle")]
    Candle,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "order_book")]
    OrderBook,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "funding_rate")]
    FundingRate,
    #[serde(rename = "open_interest")]
    OpenInterest,
    #[serde(rename = "liquidation")]
    Liquidation,
}

/// 시장 데이터 쿼리 필터
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataFilter {
    /// 심볼 필터
    pub symbols: Option<Vec<SymbolPair>>,
    /// 거래소 필터
    pub exchanges: Option<Vec<ExchangeId>>,
    /// 시작 시간 필터
    pub start_time: Option<DateTime<Utc>>,
    /// 종료 시간 필터
    pub end_time: Option<DateTime<Utc>>,
    /// 타임프레임 필터
    pub timeframe: Option<Timeframe>,
    /// 데이터 타입 필터
    pub data_type: Option<MarketDataType>,
    /// 최대 결과 수량
    pub limit: Option<u32>,
    /// 정렬 방향
    pub ascending: Option<bool>,
}

/// 캔들 데이터프레임 변환 인터페이스
pub trait CandleDataFrame {
    /// 캔들 데이터를 Polars DataFrame으로 변환
    fn to_dataframe(candles: &[Candle]) -> Result<DataFrame, polars::error::PolarsError>;
    
    /// Polars DataFrame에서 캔들 데이터로 변환
    fn from_dataframe(df: &DataFrame) -> Result<Vec<Candle>, String>;
    
    /// DataFrame을 CSV로 저장
    fn save_to_csv(df: &DataFrame, path: &str) -> Result<(), polars::error::PolarsError>;
    
    /// CSV에서 DataFrame 로드
    fn load_from_csv(path: &str) -> Result<DataFrame, polars::error::PolarsError>;
}

/// 캔들 데이터프레임 구현
pub struct CandleDataFrameImpl;

impl CandleDataFrame for CandleDataFrameImpl {
    fn to_dataframe(candles: &[Candle]) -> Result<DataFrame, polars::error::PolarsError> {
        if candles.is_empty() {
            return Err(polars::error::PolarsError::NoData("빈 캔들 배열".into()));
        }
        
        // 데이터 추출
        let mut timestamps = Vec::with_capacity(candles.len());
        let mut symbols = Vec::with_capacity(candles.len());
        let mut opens = Vec::with_capacity(candles.len());
        let mut highs = Vec::with_capacity(candles.len());
        let mut lows = Vec::with_capacity(candles.len());
        let mut closes = Vec::with_capacity(candles.len());
        let mut volumes = Vec::with_capacity(candles.len());
        
        for candle in candles {
            timestamps.push(candle.timestamp.timestamp_millis());
            symbols.push(candle.symbol.to_string());
            opens.push(candle.open);
            highs.push(candle.high);
            lows.push(candle.low);
            closes.push(candle.close);
            volumes.push(candle.volume);
        }
        
        // DataFrame 생성
        let df = DataFrame::new(vec![
            Series::new("timestamp", timestamps),
            Series::new("symbol", symbols),
            Series::new("open", opens),
            Series::new("high", highs),
            Series::new("low", lows),
            Series::new("close", closes),
            Series::new("volume", volumes),
        ])?;
        
        Ok(df)
    }
    
    fn from_dataframe(df: &DataFrame) -> Result<Vec<Candle>, String> {
        // 필요한 열이 있는지 확인
        let required_columns = vec![
            "timestamp", "symbol", "open", "high", "low", "close", "volume"
        ];
        
        for col in required_columns {
            if !df.schema().contains(col) {
                return Err(format!("DataFrame에 필수 열 '{}'이(가) 없습니다", col));
            }
        }
        
        let timestamp_col = df.column("timestamp").map_err(|e| e.to_string())?;
        let symbol_col = df.column("symbol").map_err(|e| e.to_string())?;
        let open_col = df.column("open").map_err(|e| e.to_string())?;
        let high_col = df.column("high").map_err(|e| e.to_string())?;
        let low_col = df.column("low").map_err(|e| e.to_string())?;
        let close_col = df.column("close").map_err(|e| e.to_string())?;
        let volume_col = df.column("volume").map_err(|e| e.to_string())?;
        
        let mut candles = Vec::with_capacity(df.height());
        
        for i in 0..df.height() {
            let timestamp_ms = timestamp_col.get(i).map_err(|e| e.to_string())?;
            let timestamp_ms = timestamp_ms
                .try_extract::<i64>()
                .map_err(|_| "타임스탬프가 i64 타입이 아닙니다".to_string())?;
                
            let datetime = DateTime::<Utc>::from_timestamp_millis(timestamp_ms)
                .ok_or_else(|| format!("유효하지 않은 타임스탬프: {}", timestamp_ms))?;
                
            let symbol_str = symbol_col.get(i).map_err(|e| e.to_string())?;
            let symbol_str = symbol_str
                .try_extract::<&str>()
                .map_err(|_| "심볼이 문자열 타입이 아닙니다".to_string())?;
                
            let parts: Vec<&str> = symbol_str.split('/').collect();
            if parts.len() != 2 {
                return Err(format!("유효하지 않은 심볼 형식: {}", symbol_str));
            }
            
            let symbol = SymbolPair::new(parts[0], parts[1]);
            
            let open = open_col.get(i).map_err(|e| e.to_string())?;
            let open = open
                .try_extract::<f64>()
                .map_err(|_| "시가가 f64 타입이 아닙니다".to_string())?;
                
            let high = high_col.get(i).map_err(|e| e.to_string())?;
            let high = high
                .try_extract::<f64>()
                .map_err(|_| "고가가 f64 타입이 아닙니다".to_string())?;
                
            let low = low_col.get(i).map_err(|e| e.to_string())?;
            let low = low
                .try_extract::<f64>()
                .map_err(|_| "저가가 f64 타입이 아닙니다".to_string())?;
                
            let close = close_col.get(i).map_err(|e| e.to_string())?;
            let close = close
                .try_extract::<f64>()
                .map_err(|_| "종가가 f64 타입이 아닙니다".to_string())?;
                
            let volume = volume_col.get(i).map_err(|e| e.to_string())?;
            let volume = volume
                .try_extract::<f64>()
                .map_err(|_| "거래량이 f64 타입이 아닙니다".to_string())?;
                
            candles.push(Candle {
                symbol,
                timestamp: datetime,
                open,
                high,
                low,
                close,
                volume,
            });
        }
        
        Ok(candles)
    }
    
    fn save_to_csv(df: &DataFrame, path: &str) -> Result<(), polars::error::PolarsError> {
        let mut file = std::fs::File::create(path).map_err(|e| 
            polars::error::PolarsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("파일 생성 오류: {}", e)
            ))
        )?;
        
        CsvWriter::new(&mut file)
            .has_header(true)
            .with_delimiter(b',')
            .finish(df)
    }
    
    fn load_from_csv(path: &str) -> Result<DataFrame, polars::error::PolarsError> {
        CsvReader::from_path(path)?
            .infer_schema(Some(16))
            .has_header(true)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    
    #[test]
    fn test_market_data_type() {
        let data_type = MarketDataType::Candle;
        let serialized = serde_json::to_string(&data_type).unwrap();
        assert_eq!(serialized, "\"candle\"");
        
        let deserialized: MarketDataType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, MarketDataType::Candle);
    }
    
    #[test]
    fn test_candle_dataframe_conversion() {
        let now = Utc::now();
        let candles = vec![
            Candle {
                symbol: SymbolPair::new("BTC", "USDT"),
                timestamp: now,
                open: 40000.0,
                high: 41000.0,
                low: 39500.0,
                close: 40500.0,
                volume: 100.5,
            },
            Candle {
                symbol: SymbolPair::new("BTC", "USDT"),
                timestamp: Utc.timestamp_opt(now.timestamp() + 60, 0).unwrap(),
                open: 40500.0,
                high: 40800.0,
                low: 40200.0,
                close: 40700.0,
                volume: 85.2,
            },
        ];
        
        let df = CandleDataFrameImpl::to_dataframe(&candles).unwrap();
        assert_eq!(df.height(), 2);
        assert_eq!(df.width(), 7);
        
        let reconverted = CandleDataFrameImpl::from_dataframe(&df).unwrap();
        assert_eq!(reconverted.len(), 2);
        assert_eq!(reconverted[0].open, 40000.0);
        assert_eq!(reconverted[1].close, 40700.0);
    }
} 