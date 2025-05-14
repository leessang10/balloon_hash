# @woojoo/balloon-hash-wasm

> A secure, WebAssembly-based implementation of [Balloon Hashing (RFC 9197)](https://datatracker.ietf.org/doc/html/rfc9197) for password hashing, written in Rust and compiled to WebAssembly.

---

## 🚀 Overview

This package provides a fast and secure **Balloon Hashing** algorithm, compiled from Rust to WebAssembly and usable from any JavaScript/TypeScript environment (Node.js, browsers, bundlers).

Balloon Hashing is a memory-hard password hashing algorithm introduced in 2016 as a simpler alternative to scrypt/argon2, and is defined in [RFC 9197](https://datatracker.ietf.org/doc/html/rfc9197).

---

## 🔐 Why Balloon Hash?

- ✅ RFC 9197 compliant
- ✅ Memory-hard, CPU-hard
- ✅ Resistant to GPU/ASIC attacks
- ✅ Simple implementation model
- ✅ WASM-based for performance and portability
- ✅ No native bindings required — works in browser or Node

---

## 📦 Installation

```bash
npm install @woojoo/balloon-hash-wasm
```
