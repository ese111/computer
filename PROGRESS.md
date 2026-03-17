# 프로젝트 진행 상태 보고서 (Handover)

## 📌 프로젝트 개요
- **목표**: 트랜지스터 수준(NAND)부터 시작하여 CPU, 어셈블러, OS, 고수준 언어까지 직접 구현하며 컴퓨터 구조 이해.
- **언어**: Rust (Workspace 구조)

## ✅ 완료된 단계

### Phase 1: 트랜지스터 & 논리 게이트 (`01_gates`)
- `Bit`: `Zero`, `One` 열거형 및 `Default` 구현 (기본값: `Zero`).
- `gates`: `nand`를 원시 게이트로 하여 모든 기본 논리 게이트 구현 완료.
- `Bus<const N: usize>`: N비트 버스 타입 구현 완료.

### Phase 2: 조합 회로 (`02_combinational`)
- `adders`: `Adder16` 등 산술 회로 구현 완료.
- `mux`: 다양한 채널의 멀티플렉서 구현 완료.
- `decoder`: `Decoder3to8` 구현 완료.

### Phase 3: 순차 회로 (`03_sequential`)
- `dff`: 기억의 최소 단위인 **D Flip-Flop** 구현 완료.
- `register`: 16비트 데이터를 저장하고 `load` 신호로 제어하는 **Register16** 구현 완료.
- `pc`: 명령어 주소를 관리하는 **Program Counter** 구현 및 테스트 완료. (Reset, Load, Inc 기능)

---

## 🚀 현재 진행 중인 단계: Phase 4 (ALU - Arithmetic Logic Unit)

**핵심 개념: CPU의 계산 엔진**
- 산술(Arithmetic) 연산과 논리(Logic) 연산을 수행하는 장치입니다.
- 제어 신호(AluOp)에 따라 덧셈, 뺄셈, AND, OR 등을 선택적으로 수행합니다.

### 📝 다음 작업 (Next Steps)

1.  **`src/alu.rs` 구현**:
    - `AluOp` 정의: 어떤 연산을 할지 결정하는 비트 조합.
    - 16비트 산술 연산 (Add, Sub).
    - 16비트 논리 연산 (And, Or, Not).
    - **Zero Flag (ZR)**: 결과가 0인 경우 1 출력.
    - **Negative Flag (NG)**: 결과가 음수인 경우 1 출력.

---

## 🛠 기술적 참고 사항
- `Bit` 타입에 `Default`를 구현하여 모든 순차 소자가 안전하게 `Zero`로 초기화되도록 보장함.
- `sequential` 크레이트는 `gates`와 `combinational`의 로직을 결합하여 상태를 가짐.
- 다음 단계인 ALU는 순수 조합 회로이지만, CPU의 제어 장치(Control Unit)와 밀접하게 연동될 예정.
