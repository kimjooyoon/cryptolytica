# CryptoLytica에 기여하기

CryptoLytica 프로젝트에 관심을 가져주셔서 감사합니다! 이 문서는 프로젝트에 기여하는 방법에 대한 가이드라인을 제공합니다.

## 목차

- [행동 규범](#행동-규범)
- [기여 방법](#기여-방법)
  - [버그 리포트](#버그-리포트)
  - [기능 요청](#기능-요청)
  - [코드 기여](#코드-기여)
- [개발 환경 설정](#개발-환경-설정)
- [코딩 스타일 가이드](#코딩-스타일-가이드)
- [Pull Request 프로세스](#pull-request-프로세스)
- [커뮤니티](#커뮤니티)

## 행동 규범

CryptoLytica 프로젝트는 [Contributor Covenant](https://www.contributor-covenant.org/version/2/0/code_of_conduct/)를 행동 규범으로 채택하고 있습니다. 프로젝트 참여자는 이 규범을 준수해야 합니다.

## 기여 방법

### 버그 리포트

버그를 발견하셨다면 GitHub 이슈를 통해 알려주세요. 버그 리포트 작성 시에는 다음 정보를 포함해주세요:

1. 문제 상황 요약
2. 재현 단계
3. 예상된 결과와 실제 결과
4. 스크린샷 (가능한 경우)
5. 환경 정보 (OS, 브라우저, 버전 등)

### 기능 요청

새로운 기능이나 개선점을 제안하고 싶으시다면 GitHub 이슈를 활용해주세요. 기능 요청 시에는 다음 내용을 포함해주세요:

1. 기능 요약 및 목적
2. 해당 기능이 필요한 이유와 사용 사례
3. 가능한 구현 방법 제안 (선택사항)

### 코드 기여

코드 기여는 항상 환영합니다! 다음 단계에 따라 코드 기여를 진행해주세요:

1. 프로젝트를 포크하고 로컬에 클론합니다.
2. 새로운 브랜치를 생성합니다: `git checkout -b feature/your-feature-name` 또는 `git checkout -b fix/your-bugfix-name`
3. 변경사항을 구현하고 테스트합니다.
4. 변경사항을 커밋하고 푸시합니다.
5. GitHub에서 원본 저장소로 Pull Request를 생성합니다.

## 개발 환경 설정

로컬 개발 환경을 설정하려면 다음 지침을 따르세요:

```bash
# 저장소 클론
git clone https://github.com/YourUsername/cryptolytica.git
cd cryptolytica

# 의존성 설치
make setup

# 개발 환경 실행
make dev-env

# 테스트 실행
make test
```

## 코딩 스타일 가이드

### Rust

- [Rust API 가이드라인](https://rust-lang.github.io/api-guidelines/)을 따릅니다.
- `cargo fmt` 및 `cargo clippy`를 사용하여 코드 포맷과 린팅을 확인합니다.
- 테스트 코드를 작성하고 `cargo test`로 테스트를 실행합니다.

### Go

- [Effective Go](https://golang.org/doc/effective_go) 및 [Go 코드 리뷰 코멘트](https://github.com/golang/go/wiki/CodeReviewComments)를 따릅니다.
- `go fmt` 및 `golint`를 사용하여 코드 포맷과 린팅을 확인합니다.
- 테스트 코드를 작성하고 `go test`로 테스트를 실행합니다.

## Pull Request 프로세스

1. Pull Request를 생성하기 전에 최신 `main` 브랜치와 리베이스나 머지를 수행합니다.
2. 변경사항에 관련된 테스트를 추가하고 모든 테스트가 통과하는지 확인합니다.
3. 변경사항을 문서화하고 필요한 경우 README.md를 업데이트합니다.
4. 하나의 Pull Request는 하나의 기능 또는 버그 수정에 집중합니다.
5. Pull Request 제목과 설명에 변경 내용을 명확히 기술합니다.
6. Pull Request 검토 과정에서 피드백을 받으면 필요한 수정을 진행합니다.
7. 모든 CI 검사가 통과하고 최소 1명의 메인테이너 승인을 받아야 머지됩니다.

## 커뮤니티

- GitHub 이슈: 버그 리포트, 기능 요청 및 질문
- GitHub 토론: 프로젝트에 관한 더 넓은 토론 및 아이디어 교환
- 이메일: your.email@example.com

---

이 문서는 지속적으로 개선됩니다. 기여 가이드라인에 대한 제안이나 질문이 있으시면 이슈를 통해 알려주세요.