# Balloon Hash

A secure WebAssembly-based implementation of Balloon Hashing (RFC 9197) with TypeScript support.

[한국어 문서](README.ko.md)

## Features

- WebAssembly-based implementation for high performance
- TypeScript support with type definitions
- Supports both string and Uint8Array inputs
- Configurable space, time, and parallel cost parameters
- Implements RFC 9197 specification

## Installation

```bash
npm install balloon_hash
```

## Usage

```typescript
import { BalloonHash } from 'balloon_hash';

// Create an instance
const balloonHash = new BalloonHash(
  16, // space cost (memory usage)
  20, // time cost (iterations)
  4, // parallel cost
);

// Hash with string input
const hash1 = balloonHash.hash('password', 'salt');
console.log(Buffer.from(hash1).toString('hex'));

// Hash with Uint8Array input
const passwordBuffer = new TextEncoder().encode('password');
const saltBuffer = new TextEncoder().encode('salt');
const hash2 = balloonHash.hash(passwordBuffer, saltBuffer);
console.log(Buffer.from(hash2).toString('hex'));
```

## API Reference

### `BalloonHash`

#### Constructor

```typescript
constructor(spaceCost: number, timeCost: number, parallelCost: number)
```

- `spaceCost`: Memory usage (buffer size)
- `timeCost`: Number of iterations
- `parallelCost`: Level of parallelism

#### Methods

```typescript
hash(password: string | Uint8Array, salt: string | Uint8Array): Uint8Array
```

- `password`: Password to hash (string or Uint8Array)
- `salt`: Salt value (string or Uint8Array)
- Returns: Hashed result as Uint8Array

## Development

### Prerequisites

- Node.js >= 18.0.0
- Rust and Cargo
- wasm-pack

### Commands

```bash
# Install dependencies
npm install

# Build WebAssembly and TypeScript
npm run build

# Build only WebAssembly
npm run build:wasm

# Build only TypeScript
npm run build:ts

# Run tests
npm test
```

## License

MIT License

## Author

woojoo
