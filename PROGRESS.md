# 프로젝트 진행 상태 보고서 (Handover)

## 📌 프로젝트 개요
- **목표**: 트랜지스터 수준(NAND)부터 시작하여 CPU, 어셈블러, OS, 고수준 언어까지 직접 구현하며 컴퓨터 구조 이해.
- **언어**: Rust (Workspace 구조)

## ✅ 완료된 단계

### Phase 1: 트랜지스터 & 논리 게이트 (`01_gates`)
- `Bit`: `Zero`, `One` 열거형 및 기본 변환 로직 구현.
- `gates`: `nand`를 원시 게이트로 하여 `not`, `and`, `or`, `nor`, `xor` 구현 완료.
- `Bus<const N: usize>`: N비트 버스 타입 및 `u16` 상호 변환 유틸리티 구현 완료.

### Phase 2: 조합 회로 (`02_combinational`)
- `adders`: `HalfAdder`, `FullAdder`, `Adder16`(리플 캐리 방식) 구현 및 테스트 완료.
- `mux`: `Mux`, `DMux`, `Mux16`, `Mux4way16`, `Mux8way16` 구현 완료. (하드웨어의 제어문 역할)
- `decoder`: `Decoder3to8` 구현 완료. (주소 선택 및 명령어 해석용)

---

## 🚀 현재 진행 중인 단계: Phase 3 (순차 회로)

**핵심 개념: 시간(Time)과 상태(State)의 도입**
- 조합 회로는 입력 즉시 출력이 결정되지만, 순차 회로는 클락(Clock)에 맞춰 상태를 저장하고 유지합니다.

### 현재 작업 상황
- `crates/03_sequential/Cargo.toml` 설정 완료 (`gates`, `combinational` 의존성 포함).
- `src/lib.rs` 모듈 구조 선언 완료.

### 📝 다음 작업 (Next Steps)

1.  **`src/dff.rs` 구현 완료**:
    - `DFF` (D Flip-Flop) 구조체 구현.
    - `current`, `next` 필드를 활용한 1비트 저장 로직.
    - `tick()` 메서드를 호출할 때 `next`가 `current`로 복사되는 시뮬레이션 구현.

2.  **`src/register.rs` 구현**:
    - `Register16`: 16개의 DFF를 묶어 16비트 데이터를 저장.
    - `load` 신호 구현: `load`가 1일 때만 새 값을 저장하고, 0일 때는 기존 값을 유지 (Mux 활용).

3.  **`src/pc.rs` 구현**:
    - `Program Counter`: 다음에 실행할 명령어 주소를 가리키는 특수 레지스터.
    - 기능: `inc` (1 증가), `load` (특정 주소로 점프), `reset` (0으로 초기화).
    - Phase 2의 `Adder16`을 사용하여 `inc` 로직 구현.

---

## 🛠 기술적 참고 사항
- 모든 순차 회로는 `tick()` 함수를 가집니다.
- **박자 맞추기**: 전체 시스템 테스트 시 모든 소자의 `tick()`을 동시에 호출하여 상태를 전이시킵니다.
- `gates::bus::Bus`를 적극 활용하여 16비트 처리를 수행합니다.
