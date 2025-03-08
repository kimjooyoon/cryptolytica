# CryptoLytica 테스트 주도 개발(TDD) 원칙

## 1. TDD 핵심 원칙

### 1.1 Red-Green-Refactor 사이클

CryptoLytica 프로젝트는 다음의 TDD 사이클을 철저히 준수합니다:

1. **Red**: 실패하는 테스트를 먼저 작성합니다. 
   - 구현하려는 기능의 명세를 테스트로 표현합니다.
   - 아직 구현되지 않았으므로 테스트는 실패해야 합니다.

2. **Green**: 테스트를 통과하는 가장 간단한 코드를 작성합니다.
   - 이 단계에서는 코드의 품질보다 테스트 통과에 집중합니다.
   - 필요한 최소한의 코드만 작성합니다.

3. **Refactor**: 테스트를 통과하는 코드를 개선합니다.
   - 중복 제거, 가독성 향상, 성능 최적화 등을 수행합니다.
   - 리팩토링 후에도 모든 테스트가 통과해야 합니다.

### 1.2 테스트 우선 원칙

- **테스트 없는 코드 금지**: 모든 프로덕션 코드는 관련 테스트가 먼저 작성된 후에만 구현합니다.
- **테스트 주도 설계**: 테스트를 먼저 작성함으로써 더 나은 API와 설계를 유도합니다.
- **작은 단위로 진행**: 한 번에 하나의 기능만 테스트하고 구현합니다.

### 1.3 테스트 실패 시 개발 중단 정책

- 테스트가 실패하면 **모든 개발 작업을 즉시 중단**하고 실패 원인을 해결합니다.
- 새로운 테스트를 작성하기 전에 기존 테스트가 모두 통과해야 합니다.
- 실패한 테스트는 다음 우선순위로 처리합니다:
  1. 테스트 코드 자체의 오류 확인
  2. 구현 코드의 오류 수정
  3. 필요시 테스트 또는 명세 조정

## 2. 테스트 작성 가이드라인

### 2.1 테스트 구조

모든 테스트는 다음 구조를 따릅니다:

- **Given**: 테스트 준비 단계 (설정, 초기화)
- **When**: 테스트할 동작 실행
- **Then**: 결과 검증

### 2.2 테스트 명명 규칙

- Rust 테스트:
  ```rust
  #[test]
  fn should_행위_when_상황() {
      // 테스트 내용
  }
  ```

- Go 테스트:
  ```go
  func TestShouldDoSomethingWhenSomeCondition(t *testing.T) {
      // 테스트 내용
  }
  ```

### 2.3 FIRST 원칙 준수

모든 테스트는 다음 FIRST 원칙을 준수해야 합니다:

- **Fast**: 테스트는 빠르게 실행되어야 합니다.
- **Isolated**: 테스트는 독립적이어야 하며 다른 테스트에 의존해서는 안 됩니다.
- **Repeatable**: 테스트는 환경에 관계없이 항상 같은 결과를 보여야 합니다.
- **Self-validating**: 테스트는 스스로 성공/실패를 판단할 수 있어야 합니다.
- **Timely**: 테스트는 프로덕션 코드 구현 전에 작성되어야 합니다.

## 3. 테스트 종류 및 범위

### 3.1 단위 테스트

- 개별 함수, 메서드, 클래스의 동작을 검증합니다.
- 외부 의존성은 모의 객체(Mock)로 대체합니다.
- 애그리게이트 내부 로직과 도메인 규칙을 검증합니다.

### 3.2 통합 테스트

- 여러 컴포넌트 간의 상호작용을 검증합니다.
- 바운디드 컨텍스트 간 통합을 검증합니다.
- 실제 의존성을 사용하거나 통합 테스트용 스텁을 활용합니다.

### 3.3 시스템 테스트

- 전체 시스템의 종단간(End-to-End) 동작을 검증합니다.
- 주요 사용자 시나리오와 워크플로우를 검증합니다.

## 4. 언어별 테스트 프레임워크

### 4.1 Rust 테스트 프레임워크

- Rust의 내장 테스트 프레임워크 사용
- 추가 크레이트:
  - `mockall`: 목킹 라이브러리
  - `rstest`: 파라미터화된 테스트 지원
  - `tokio-test`: 비동기 코드 테스트

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_calculate_correctly() {
        // Given
        let input = 5;
        
        // When
        let result = calculate(input);
        
        // Then
        assert_eq!(result, 10);
    }
}
```

### 4.2 Go 테스트 프레임워크

- Go의 내장 `testing` 패키지 사용
- 추가 라이브러리:
  - `testify`: 어설션 및 목킹 지원
  - `gomock`: 인터페이스 목킹
  - `httptest`: HTTP 테스트

```go
func TestCalculate(t *testing.T) {
    // Given
    input := 5
    
    // When
    result := Calculate(input)
    
    // Then
    assert.Equal(t, 10, result)
}
```

## 5. 테스트 자동화 및 CI/CD 통합

### 5.1 로컬 테스트 실행

```bash
# 전체 테스트 실행
make test

# 특정 컴포넌트 테스트 실행
make test-component COMPONENT=exchange

# 빠른 테스트만 실행 (통합/시스템 테스트 제외)
make test-fast
```

### 5.2 CI/CD 파이프라인 통합

- 모든 PR은 테스트 통과 후에만 머지 가능
- 테스트 커버리지 임계값: 80% 이상 유지
- 주기적인 성능 테스트 및 벤치마크 실행

## 6. 테스트 데이터 관리

### 6.1 테스트 데이터 생성

- 가능한 테스트 내에서 데이터 생성
- 공통 데이터는 팩토리 함수나 테스트 헬퍼 사용
- 대용량 데이터는 별도 파일로 관리

### 6.2 외부 의존성 처리

- 실제 서비스를 테스트해야 하는 경우 테스트 컨테이너 활용
- 외부 API는 모의 서버(Mock Server)로 대체
- 데이터베이스는 인메모리 구현 또는 테스트 컨테이너 사용

## 7. 테스트 실패 대응 절차

### 7.1 테스트 실패 시 조치

1. 실패한 테스트 로그 확인 및 원인 분석
2. 로컬 환경에서 실패 재현
3. 문제 해결 및 수정
4. 회귀 테스트 실행으로 다른 기능 영향 없음 확인

### 7.2 순환 의존성 및 복잡한 테스트 처리

- 순환 의존성은 설계 문제로 간주하고 리팩토링
- 복잡한 통합 테스트는 작은 단위로 분해
- 필요시 아키텍처 개선 고려

## 8. 특수 사례 테스트

### 8.1 비동기 및 병렬 처리 테스트

- Rust: `tokio-test` 및 `futures` 크레이트 활용
- Go: 고루틴 및 채널 테스트 패턴 사용

### 8.2 암호화폐 특화 테스트

- 시장 데이터 정확성 테스트
- 거래소 API 에러 처리 테스트
- 네트워크 지연 및 장애 복구 테스트

## 9. 효과적인 TDD 실천을 위한 팁

- 작은 단위로 진행하며 점진적으로 복잡성 추가
- 테스트 코드도 프로덕션 코드와 동일한 품질 기준 적용
- 테스트 코드 리팩토링도 중요하게 다룸
- 정기적인 TDD 세션 및 페어 프로그래밍 장려

## 10. 참고 자료

- [Test-Driven Development by Example (Kent Beck)](https://www.amazon.com/Test-Driven-Development-Kent-Beck/dp/0321146530)
- [Growing Object-Oriented Software, Guided by Tests](https://www.amazon.com/Growing-Object-Oriented-Software-Guided-Tests/dp/0321503627)
- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Go Testing Documentation](https://golang.org/pkg/testing/)