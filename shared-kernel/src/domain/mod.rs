//! 도메인 레이어 - 공통 도메인 모델 정의
//!
//! 이 모듈은 여러 바운디드 컨텍스트에서 공유되는 핵심 도메인 개념들을 정의합니다.
//! Value Object, Entity, Domain Event 등 공통으로 사용되는 도메인 객체들이 포함됩니다.

pub mod model;
pub mod repository;
pub mod service;
pub mod value_object;
pub mod aggregate; 