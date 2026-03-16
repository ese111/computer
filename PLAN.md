# 컴퓨터 기본 원리 학습 프로젝트 - 구현 플랜

## Context
컴퓨터의 기본 지식을 학습하기 위해 Rust로 트랜지스터부터 시작해 간단한 OS와 프로그래밍 언어까지 단계적으로 구현하는 프로젝트. 각 단계가 이전 단계 위에 쌓이는 bottom-up 방식으로, 실제 컴퓨터의 구조를 체험적으로 학습한다.

## 프로젝트 구조

Cargo workspace로 단계별 crate 구성:

```
computer/
  Cargo.toml              (workspace root)
  README.md
  crates/
    01_gates/             Phase 1: 트랜지스터 & 논리 게이트
    02_combinational/     Phase 2: 조합 회로
    03_sequential/        Phase 3: 순차 회로
    04_alu/               Phase 4: ALU
    05_memory/            Phase 5: RAM & ROM
    06_cpu/               Phase 6: CPU
    07_assembler/         Phase 7: 어셈블러
    08_os/                Phase 8: 간단한 OS
    09_lang/              Phase 9: 프로그래밍 언어
```

---

## Phase 1: 트랜지스터 & 논리 게이트 (`01_gates`)

**구현 내용:**
- `Bit` 타입 (Zero/One enum)
- `nand` 함수를 유일한 원시 게이트로 정의
- nand로부터 모든 게이트 파생: `not`, `and`, `or`, `nor`, `xor`
- `Bus<const N: usize>` 타입 (N비트 버스)

**핵심 학습:** 모든 논리 회로는 NAND 하나로 만들 수 있다

**파일:** `bit.rs`, `gates.rs`, `bus.rs`

---

## Phase 2: 조합 회로 (`02_combinational`)

**구현 내용:**
- 반가산기(Half Adder), 전가산기(Full Adder), 16비트 리플캐리 가산기
- 멀티플렉서(Mux), 디멀티플렉서(Demux), 디코더

**핵심 학습:** 비트 수준의 덧셈, MUX = 하드웨어 if/else

**의존:** Phase 1 게이트들로만 구성

---

## Phase 3: 순차 회로 (`03_sequential`)

**구현 내용:**
- SR Latch, D Flip-Flop (current/next 2단계 모델)
- Register<N>, Program Counter (inc/load/reset)
- Clock 구조체

**핵심 학습:** 순수 함수 → 상태를 가진 회로로의 전환. `tick()`에서만 상태 변경.

```rust
pub struct DFF {
    current: Bit,
    next: Bit,
}
impl DFF {
    pub fn tick(&mut self) { self.current = self.next; }
}
```

---

## Phase 4: ALU (`04_alu`)

**구현 내용:**
- 16비트 Add, Sub(2의 보수), AND, OR, NOT
- Zero/Negative 플래그
- AluOp enum으로 연산 선택

**핵심 학습:** 2의 보수, 조건 플래그가 점프 명령어를 구동

---

## Phase 5: 메모리 (`05_memory`)

**구현 내용:**
- RAM8 → RAM64 → RAM512 → RAM4K (계층적 구성)
- ROM (프로그램 저장용)
- Memory Map (주소 범위별 RAM/ROM 라우팅)

**핵심 학습:** 메모리 주소 디코딩, 읽기는 조합적/쓰기는 순차적

**실용 참고:** 큰 RAM은 `Vec<u16>` 기반 FastRam도 병행 제공

---

## Phase 6: CPU (`06_cpu`) ⭐ 핵심 통합 지점

**구현 내용:**
- 16비트 ISA 정의 (~16개 명령어):
  - `NOP, LOAD, STORE, MOV, MOVI, ADD, SUB, AND, OR, NOT, CMP, JMP, JEQ, JNE, CALL, RET`
- 명령어 디코더, 제어 유닛, 데이터패스
- Fetch-Decode-Execute 사이클

**핵심 학습:** 폰 노이만 아키텍처, 저장 프로그램 개념

**테스트:** raw `[u16]` 배열 프로그램을 ROM에 로드 → CPU 실행 → 레지스터/메모리 검증

---

## Phase 7: 어셈블러 (`07_assembler`)

**구현 내용:**
- Lexer → Parser → Symbol Table → Code Generator
- 2-pass 어셈블리 (1차: 레이블 수집, 2차: 코드 생성)
- CLI: `.asm` 파일 → 바이너리

**핵심 학습:** 니모닉과 기계어의 직접 대응, 언어 처리 첫 경험

**통합 테스트:** 어셈블 → CPU ROM 로드 → 실행 → 결과 검증

---

## Phase 8: 간단한 OS (`08_os`)

**구현 내용:**
- 부트로더 개념 (시스템 초기화)
- SYSCALL 명령어, 시스템 콜 핸들러
- 프로세스 테이블 & 라운드로빈 스케줄러 (2~3개 프로세스)
- 고정 파티션 메모리 관리
- 메모리 맵 I/O (0xFFFF에 쓰면 문자 출력)

**핵심 학습:** OS는 다른 프로그램을 관리하는 프로그램, 컨텍스트 스위칭

**범위:** 최소한으로 유지 — 파일시스템/가상메모리 없음

---

## Phase 9: 프로그래밍 언어 (`09_lang`)

**구현 내용:**
- Lexer → Parser(재귀 하강) → Type Checker → Code Generator(→ 어셈블리)
- 지원 문법: 변수, if/else, while, 함수, print

```
let x = 5;
fn add(a, b) { return a + b; }
print(add(x, 3));
```

**핵심 학습:** 소스코드 → 토큰 → AST → 어셈블리 → 기계어 → CPU 실행의 전체 파이프라인

---

## 구현 순서 & 예상 기간

| Phase | 크레이트 | 예상 기간 | 마일스톤 |
|-------|---------|----------|---------|
| 1 | `01_gates` | 1~2일 | 모든 게이트 truth table 테스트 통과 |
| 2 | `02_combinational` | 2~3일 | 16비트 가산기 동작 |
| 3 | `03_sequential` | 2~3일 | 레지스터 tick 기반 저장/읽기 |
| 4 | `04_alu` | 2~3일 | 모든 연산 + 플래그 정상 |
| 5 | `05_memory` | 2~3일 | RAM 읽기/쓰기, 메모리맵 라우팅 |
| 6 | `06_cpu` | 5~7일 | 수작업 프로그램 실행 성공 |
| 7 | `07_assembler` | 3~5일 | 피보나치 어셈블 & 실행 |
| 8 | `08_os` | 5~7일 | 2개 프로세스 타임셰어링 |
| 9 | `09_lang` | 5~7일 | 피보나치 컴파일 → 어셈블 → 실행 |

**총 예상: 약 4~8주**

## 검증 방법

- 매 Phase마다 `cargo test` 통과
- Phase 6 이후: CPU 상태 출력으로 시각적 디버깅
  ```
  TICK 42 | PC=0x000A | R0=5 R1=3 R2=8 | FLAGS: Z=0 N=0 | INSTR: ADD R2, R0, R1
  ```
- Phase 7+9 통합: 소스코드 → ... → CPU 실행까지 end-to-end 검증
