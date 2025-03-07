# CryptoLytica

<div align="center">
  <img src="https://via.placeholder.com/200x200?text=CryptoLytica" alt="CryptoLytica Logo" width="200" height="200">
</div>

<div align="center">
  <strong>고성능 암호화폐 데이터 수집 및 분석을 위한 오픈소스 플랫폼</strong>
</div>

<div align="center">
  <img src="https://img.shields.io/badge/Rust-1.79.0-orange" alt="Rust">
  <img src="https://img.shields.io/badge/Go-1.22-blue" alt="Go">
  <img src="https://img.shields.io/badge/License-Apache%202.0-green" alt="License">
</div>

<br />

## 🚀 프로젝트 소개

CryptoLytica는 암호화폐 시장 데이터의 수집, 처리, 분석을 위한 종합 플랫폼입니다. Rust와 Go의 강점을 결합하여 고성능 데이터 처리와 확장 가능한 서비스를 구현했습니다. 여러 거래소와 블록체인의 데이터를 표준화하고 통합하여 복잡한 분석 작업을 효율적으로 수행합니다.

## ✨ 핵심 기능

- **다중 거래소 데이터 통합**: 주요 거래소 API 연동과 실시간 데이터 수집
- **온체인 데이터 분석**: 블록체인 트랜잭션 패턴 분석 및 주요 지갑 활동 추적
- **포트폴리오 분석 및 최적화**: 암호화폐 포트폴리오 성과 측정 및 최적 배분 계산
- **고급 시장 분석**: 변동성 패턴, 추세 분석, 시장 감정 지표 생성
- **머신러닝 통합**: 가격 패턴 예측, 이상 탐지 및 리스크 모델링

## 🔗 생태계 통합

CryptoLytica는 다음 프로젝트들과 함께 통합 금융 데이터 생태계의 일부로 작동합니다:

- **[go_fi_chart](https://github.com/kimjooyoon/go_fi_chart)**: 전통 금융 자산 데이터 처리 및 분석 백엔드
- **[flutter_fi_chart](https://github.com/kimjooyoon/flutter_fi_chart)**: 금융 데이터 시각화 및 사용자 인터페이스

이 통합 생태계는 전통 금융과 암호화폐를 포괄하는 완전한 금융 데이터 솔루션을 제공합니다. 
자세한 내용은 [ECOSYSTEM.md](ECOSYSTEM.md) 문서를 참조하세요.

```
┌───────────────────┐      ┌───────────────────┐
│                   │      │                   │
│    go_fi_chart    │◄────►│   CryptoLytica    │
│  (전통 금융 자산)   │      │    (암호화폐)      │
│                   │      │                   │
└─────────┬─────────┘      └────────┬──────────┘
          │                          │
          │         ┌────────────────┘
          │         │
          ▼         ▼
┌───────────────────────────┐
│                           │
│      flutter_fi_chart     │
│   (통합 사용자 인터페이스)   │
│                           │
└───────────────────────────┘
```

## 🔧 기술 스택

- **Rust**: 코어 데이터 수집/처리 엔진, 시계열 분석 라이브러리
- **Go**: API 서비스 레이어, 분산 시스템 조정
- **ClickHouse/TimescaleDB**: 시계열 데이터 저장
- **Apache Kafka/Redpanda**: 이벤트 스트리밍 
- **gRPC**: 서비스 간 통신
- **Docker/Kubernetes**: 배포 및 확장

## 📁 프로젝트 구조

```
crypto-lytica/
├── core/ (Rust)
│   ├── collectors/ (거래소 및 블록체인 데이터 수집기)
│   ├── processors/ (데이터 처리 파이프라인)
│   └── storage/ (시계열 데이터 스토리지 레이어)
├── analytics/ (Rust)
│   ├── statistics/ (통계 분석 도구)
│   ├── patterns/ (패턴 인식 및 시장 분석)
│   ├── ml/ (머신러닝 통합)
│   └── visualization/ (차트 및 시각화 데이터 생성)
├── api/ (Go)
│   ├── rest/ (RESTful API)
│   ├── ws/ (WebSocket 실시간 데이터)
│   └── query/ (고급 쿼리 처리 엔진)
├── plugins/ (플러그인 시스템)
│   ├── exchanges/ (거래소 커넥터)
│   ├── blockchains/ (블록체인 데이터 소스)
│   └── models/ (분석 모델 플러그인)
└── examples/ (예제 코드 및 사용 시나리오)
```

## 🏁 시작하기

### 필수 요구사항

- Rust 1.79.0 이상
- Go 1.22 이상
- Docker 24.0.0 이상
- Docker Compose 2.24.0 이상

### 설치 방법

```bash
# 저장소 클론
git clone https://github.com/kimjooyoon/cryptolytica.git
cd cryptolytica

# 의존성 설치
make setup

# 개발 환경 실행
make dev-env

# 테스트 실행
make test
```

## 🔄 로드맵

- **Phase 1**: 기본 인프라 (주요 거래소 API 연동, 데이터 모델 구현)
- **Phase 2**: 분석 엔진 (시계열 분석, 패턴 인식, 포트폴리오 분석)
- **Phase 3**: API 서비스 (RESTful API, WebSocket 스트림)
- **Phase 4**: 확장성 (플러그인 시스템, 온체인 데이터 통합)
- **Phase 5**: 고급 기능 (머신러닝 모델, 예측 시스템)
- **Phase 6**: 통합 생태계 (go_fi_chart 및 flutter_fi_chart와의 통합)

## 👥 기여하기

기여는 언제나 환영합니다! 다음과 같은 방법으로 프로젝트에 참여하실 수 있습니다:

1. 이슈 생성 및 버그 리포트
2. 기능 요청 및 아이디어 제안
3. Pull Request 제출
4. 문서 개선

자세한 내용은 [CONTRIBUTING.md](CONTRIBUTING.md)를 참조해주세요.

## 📄 라이센스

이 프로젝트는 Apache License 2.0 라이센스 하에 배포됩니다.
자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

## 📢 연락처

- 이슈 트래커: [GitHub Issues](https://github.com/kimjooyoon/cryptolytica/issues)
- 이메일: your.email@example.com

## ⭐ 스타 히스토리

[![Star History Chart](https://api.star-history.com/svg?repos=kimjooyoon/cryptolytica&type=Date)](https://star-history.com/#kimjooyoon/cryptolytica&Date)