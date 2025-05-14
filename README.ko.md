# Balloon Hash

TypeScript를 지원하는 WebAssembly 기반 Balloon 해싱(RFC 9197) 구현체입니다.

[English Documentation](README.md)

## 특징

- WebAssembly 기반 구현으로 높은 성능 제공
- TypeScript 지원 및 타입 정의 포함
- 문자열과 Uint8Array 입력 모두 지원
- 공간, 시간, 병렬 처리 비용 매개변수 설정 가능
- RFC 9197 명세 준수

## 설치

```bash
npm install balloon_hash
```

## 사용법

```typescript
import { BalloonHash } from 'balloon_hash';

// 인스턴스 생성
const balloonHash = new BalloonHash(
  16, // 공간 비용 (메모리 사용량)
  20, // 시간 비용 (반복 횟수)
  4, // 병렬 처리 비용
);

// 문자열 입력으로 해시 생성
const hash1 = balloonHash.hash('password', 'salt');
console.log(Buffer.from(hash1).toString('hex'));

// Uint8Array 입력으로 해시 생성
const passwordBuffer = new TextEncoder().encode('password');
const saltBuffer = new TextEncoder().encode('salt');
const hash2 = balloonHash.hash(passwordBuffer, saltBuffer);
console.log(Buffer.from(hash2).toString('hex'));
```

## API 참조

### `BalloonHash`

#### 생성자

```typescript
constructor(spaceCost: number, timeCost: number, parallelCost: number)
```

- `spaceCost`: 메모리 사용량 (버퍼 크기)
- `timeCost`: 반복 횟수
- `parallelCost`: 병렬 처리 수준

#### 메서드

```typescript
hash(password: string | Uint8Array, salt: string | Uint8Array): Uint8Array
```

- `password`: 해시할 비밀번호 (문자열 또는 Uint8Array)
- `salt`: 솔트 값 (문자열 또는 Uint8Array)
- 반환값: 해시된 결과 (Uint8Array)

## 개발

### 필수 요구사항

- Node.js >= 18.0.0
- Rust와 Cargo
- wasm-pack

### 명령어

```bash
# 의존성 설치
npm install

# WebAssembly와 TypeScript 빌드
npm run build

# WebAssembly만 빌드
npm run build:wasm

# TypeScript만 빌드
npm run build:ts

# 테스트 실행
npm test
```

## 라이선스

MIT 라이선스

## 작성자

woojoo
