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
- `mux`: `Mux8Way16`, `DMux8Way` 등 다양한 채널의 멀티플렉서/디멀티플렉서 구현 완료.
- `decoder`: `Decoder3to8` 구현 완료.

### Phase 3: 순차 회로 (`03_sequential`)
- `dff`: 기억의 최소 단위인 **D Flip-Flop** 구현 완료.
- `register`: 16비트 데이터를 저장하고 `load` 신호로 제어하는 **Register16** 구현 완료.
- `pc`: 명령어 주소를 관리하는 **Program Counter** 구현 및 테스트 완료.

### Phase 4: ALU (Arithmetic Logic Unit) (`04_alu`)
- **ALU 구현 완료**: 6개의 제어 비트를 조합하여 다양한 산술/논리 연산 수행.
- **플래그 생성**: 결과 상태(`zr`, `ng`) 비트 출력 기능 구현.

### Phase 5: 메모리 (Memory) (`05_memory`)
- **RAM 구현**: `RAM8` -> `RAM64`로 이어지는 계층적 메모리 구조 구현 완료. (DMux/Mux 기반 주소 지정)
- **ROM 구현**: 프로그램 명령어를 담는 읽기 전용 메모리 시뮬레이션 구현 완료.
- **Memory Map 구현**: RAM, Screen, Keyboard 등 서로 다른 장치를 하나의 주소 체계로 통합 관리하는 로직 구현 완료.

---

## 🚀 현재 진행 중인 단계: Phase 6 (CPU - Central Processing Unit) ⭐ 핵심 통합 지점

**핵심 개념: 폰 노이만 아키텍처의 완성**
- 지금까지 만든 ALU, Register, PC, Memory를 하나로 통합합니다.
- 명령어(Instruction)를 읽어와서 해석(Decode)하고 실행(Execute)하는 사이클을 구현합니다.

### 📝 다음 작업 (Next Steps)

1.  **`src/cpu.rs` 구현**:
    - `A-Instruction` (주소 설정) 및 `C-Instruction` (연산 및 제어) 해석 로직.
    - 데이터패스 구축: 레지스터와 ALU 사이의 데이터 흐름 연결.
    - 제어 유닛: 명령어 비트에 따라 ALU 제어 신호 및 레지스터 `load` 신호 생성.
2.  **`src/computer.rs` 구현**:
    - CPU와 MemoryMap, ROM을 연결하여 전체 시스템 완성.
3.  **기계어 프로그램 실행**:
    - 하드코딩된 기계어 배열을 ROM에 로드하여 실제 연산이 일어나는지 검증.

---

## 🛠 기술적 참고 사항
- `02_combinational`에 `dmux4way`, `dmux8way`를 추가하여 대규모 메모리 주소 지정 로직을 지원함.
- `memory` 크레이트는 CPU가 외부 세계(화면, 키보드)와 소통하는 관문 역할을 함.
- Phase 6은 프로젝트에서 가장 복잡한 단계이며, 모든 부품이 정교하게 맞물려야 함.
