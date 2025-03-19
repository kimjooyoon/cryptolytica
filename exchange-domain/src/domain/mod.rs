//! 거래소 도메인 레이어
//!
//! 이 모듈은 거래소 도메인의 핵심 개념을 정의합니다.
//! DDD 원칙에 따라 엔티티, 값 객체, 도메인 서비스, 리포지토리 인터페이스, 
//! 도메인 이벤트 등을 포함합니다.

pub mod model;
pub mod service;
pub mod repository;
pub mod event;
pub mod exception; 