# Hash456 Specification

## 1. Overview
Hash456 is a 456-bit cryptographic hash function based on the Sponge Construction.

## 2. Parameters
- State Size: 512 bits
- Rate (r): 256 bits
- Capacity (c): 256 bits
- Rounds: 24
- S-Box: 8-bit AES-like Substitution Box
- Diffusion: Simplified MDS-like XOR-Shift

## 3. Padding
Input is padded using `10*1` rule to multiple of Rate (256 bits).

## 4. Security Claims
- Collision Resistance: 128-bit (theoretical limit c/2)
- Preimage Resistance: 256-bit (theoretical limit c)
