//! 값 객체(Value Object) 모듈
//!
//! 이 모듈은 여러 도메인에서 공유되는 값 객체들을 정의합니다.
//! 값 객체는 식별자가 없고 속성에 의해 정의되는 불변 객체입니다.

pub mod money;
pub mod symbol;
pub mod timeframe;
pub mod timestamp;

pub use money::Money;
pub use symbol::SymbolPair;
pub use timeframe::Timeframe;
pub use timestamp::Timestamp; 