# Hash456-Research

### Research-grade 456-bit sponge hash function (educational & analysis only)

Sponge construction · 512-bit state · AES-like S-box · Implemented in Rust

[![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)](https://www.rust-lang.org/)
[![Purpose](https://img.shields.io/badge/Purpose-Research%20%2F%20Educational-yellow)](#disclaimer)

> **Not a production cryptographic standard.** For learning, analysis, and experimentation only.

---

## Overview

Hash456 is a custom cryptographic hash design based on the **sponge construction**.

| Parameter | Value |
|-----------|-------|
| Output size | 456 bits |
| State size | 512 bits |
| Rate (r) | 256 bits |
| Capacity (c) | 256 bits |
| Rounds | 24 |
| S-Box | 8-bit AES-like |
| Diffusion | Simplified MDS-like XOR-Shift |
| Padding | `10*1` to rate boundary |

### Security claims (theoretical)

- Collision resistance: ~128-bit (c/2)
- Preimage resistance: ~256-bit (c)

See [SPEC.md](SPEC.md) for the full specification.

---

## Build & use

```bash
git clone https://github.com/sayan9168/Hash456-Research.git
cd Hash456-Research

cargo build --release
cargo test
```

Example tooling lives under `examples/` and `tools/`.

---

## Layout

```text
src/lib.rs              # Core hash implementation
examples/               # Research helpers
tools/gen_sbox.py       # S-box generation helper
SPEC.md                 # Algorithm specification
SECURITY.md             # Security notes
```

---

## Disclaimer

This is a **research project**. It has not undergone formal cryptanalysis or third-party audit.  
Do **not** use Hash456 to protect real secrets, passwords, or production systems.

---

## License

See [LICENSE](LICENSE).

Author: [Sayan Mahata](https://github.com/sayan9168)
