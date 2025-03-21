# CryptoLytica

CryptoLytica는 암호화폐 거래 및 분석을 위한 종합 플랫폼입니다. 이 프로젝트는 도메인 주도 설계(DDD) 원칙과 Factor 12 원칙에 따라 구성되었습니다.

## 아키텍처

CryptoLytica는 다음과 같은 도메인 중심 아키텍처로 구성되어 있습니다:

### 도메인 모듈

- **shared-kernel**: 모든 도메인에서 공통으로 사용되는 타입, 유틸리티, 에러 처리 등
- **market-domain**: 시장 데이터 관련 핵심 도메인 로직
- **exchange-domain**: 거래소 연동 관련 핵심 도메인 로직
- **trading-domain**: 트레이딩 전략 관련 핵심 도메인 로직
- **portfolio-domain**: 포트폴리오 및 자산 관리 관련 도메인 로직
- **notification-domain**: 알림 및 이벤트 관련 도메인 로직
- **analytics-domain**: 데이터 분석 및 리포팅 관련 도메인 로직

### 인프라스트럭처 및 인터페이스

- **infrastructure**: 실제 기술적 구현 (데이터베이스, 외부 API, 메시징 등)
- **api-gateway**: 외부 시스템과의 통신을 위한 인터페이스

## 아키텍처 원칙

이 프로젝트는 다음과 같은 아키텍처 원칙을 따릅니다:

1. **명확한 도메인 경계**: 각 도메인은 자신의 책임 영역에 대한 명확한 경계를 갖습니다.
2. **느슨한 결합**: 도메인 간 통신은 인터페이스와 이벤트를 통해 이루어집니다.
3. **높은 응집도**: 각 도메인 내에서는 관련 기능이 함께 응집되어 있습니다.
4. **인프라스트럭처 독립성**: 도메인 로직은 인프라스트럭처 구현에 의존하지 않습니다.
5. **의존성 규칙**: 저수준 모듈이 고수준 모듈에 의존하지 않습니다. 모든 의존성은 명시적으로 관리됩니다.

## Factor 12 특성

이 프로젝트는 클라우드 네이티브 애플리케이션을 위한 Factor 12 원칙을 적용합니다:

1. **코드베이스**: 각 도메인은 별도의 코드베이스로 관리되며, 하나의 저장소에서 통합됩니다.
2. **의존성**: 각 도메인은 자신의 의존성을 명시적으로 선언합니다.
3. **구성**: 구성은 환경에서 분리되어 관리됩니다.
4. **백엔드 서비스**: 데이터베이스, 메시징 시스템 등은 연결 가능한 리소스로 취급됩니다.
5. **빌드, 릴리스, 실행**: 빌드, 릴리스, 실행 단계가 엄격히 분리됩니다.
6. **프로세스**: 서비스는 상태를 공유하지 않는 프로세스로 실행됩니다.
7. **포트 바인딩**: 서비스는 포트 바인딩을 통해 노출됩니다.
8. **동시성**: 도메인을 수직적으로 확장하는 대신 수평적 확장을 선호합니다.
9. **폐기 용이성**: 빠른 시작과 우아한 종료를 지원합니다.
10. **개발/운영 일치**: 개발 환경과 운영 환경의 차이를 최소화합니다.
11. **로그**: 로그는 이벤트 스트림으로 취급됩니다.
12. **관리 프로세스**: 관리 작업은 일회성 프로세스로 실행됩니다.

## 도메인 작업 가이드라인

각 도메인 모듈에서 작업할 때 다음 가이드라인을 따라주세요:

1. **책임 영역 존중**: 각 도메인은 자신의 책임 영역에 대한 작업만 수행해야 합니다.
2. **인터페이스 기반 통신**: 다른 도메인과의 통신은 명확히 정의된 인터페이스를 통해서만 이루어져야 합니다.
3. **이벤트 기반 통신**: 도메인 간 결합도를 낮추기 위해 이벤트 기반 통신을 사용합니다.
4. **인프라스트럭처 분리**: 도메인 로직은 인프라스트럭처 구현에 의존하지 않아야 합니다.
5. **테스트 가능성**: 모든 도메인 로직은 외부 의존성 없이 테스트 가능해야 합니다.
6. **문서화**: 각 도메인의 공개 API와 이벤트는 명확하게 문서화되어야 합니다.

## 시작하기

```bash
# 프로젝트 클론
git clone https://github.com/kimjooyoon/cryptolytica.git
cd cryptolytica

# 의존성 설치 및 빌드
cargo build

# 테스트 실행
cargo test
```

## 라이선스

Apache License 2.0
