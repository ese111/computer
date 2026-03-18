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
- `pc`: 명령어 주소를 관리하는 **Program Counter** 구현 및 테스트 완료.

### Phase 4: ALU (Arithmetic Logic Unit) (`04_alu`)
- **ALU 구현 완료**: 6개의 제어 비트(`zx, nx, zy, ny, f, no`)를 조합하여 18가지 이상의 연산 수행 가능.
- **플래그 생성**: 결과가 0인지(`zr`), 음수인지(`ng`) 판단하는 상태 비트 출력 기능 구현.
- **유틸리티**: 16비트 단위의 AND, NOT 연산 로직 포함.

---

## 🚀 현재 진행 중인 단계: Phase 5 (메모리 - Memory)

**핵심 개념: 데이터의 대량 저장과 주소 지정(Addressing)**
- 레지스터를 계층적으로 쌓아 올려 거대한 RAM 공간을 만듭니다.
- `RAM8` -> `RAM64` -> `RAM512` -> `RAM4K` -> `RAM16K` 순으로 확장합니다.

### 📝 다음 작업 (Next Steps)

1.  **`src/ram.rs` 구현**:
    - `RAM8`: 8개의 `Register16`과 주소 선택을 위한 `DMux8Way`, `Mux8Way16` 결합.
    - `RAM64`: 8개의 `RAM8`을 묶어 더 큰 주소 공간 확보.
2.  **`src/rom.rs` 구현**:
    - 프로그램 코드를 읽기 전용으로 저장하는 ROM 시뮬레이션.
3.  **`src/memory_map.rs` 구현**:
    - 특정 주소 범위에 따라 RAM, Screen, Keyboard 등으로 데이터를 라우팅하는 로직.

---

## 🛠 기술적 참고 사항
- ALU는 순수 조합 회로로 구현되었으며, `sequential`의 레지스터들과 결합하여 다음 단계의 CPU 데이터패스를 형성함.
- `PROGRESS.md`를 통해 각 단계별 의존성과 핵심 성취를 지속적으로 추적 중.
