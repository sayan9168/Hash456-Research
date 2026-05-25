//! Hash456: 456-bit Sponge-based Hash & MAC (Research Prototype)
//! ⚠️ WARNING: Educational/Analysis use only. Not production-safe.

use digest::{Update, Reset, FixedOutput, OutputSizeUser};
use typenum::U57;

// ─────────────────────────────────────────────────────────────
// CONSTANTS
// ─────────────────────────────────────────────────────────────
const STATE_SIZE: usize = 64; // 512 bits
const RATE: usize = 32;       // 256 bits (absorb/squeeze rate)
const CAPACITY: usize = 32;   // 256 bits (security margin)
const ROUNDS: usize = 24;     // Permutation rounds

// AES-Style 8-bit S-Box (Non-Linearity)
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

// ─────────────────────────────────────────────────────────────
// CORE STRUCT
// ─────────────────────────────────────────────────────────────
#[derive(Clone, Default)]
pub struct Hash456 {
    state: [u8; STATE_SIZE],
    buffer: Vec<u8>, // Handles unaligned updates
}

impl Hash456 {
    pub fn new() -> Self {
        Self {
            state: [0u8; STATE_SIZE],
            buffer: Vec::new(),
        }
    }
    /// Permutation Layer (SPN: SubBytes → Shift → Mix → AddRC)
    fn permutation(&mut self, round_idx: usize) {
        // 1. SubBytes
        for i in 0..STATE_SIZE {
            self.state[i] = SBOX[self.state[i] as usize];
        }
        // 2. Rotate State (Breaks word alignment)
        let rot = round_idx % 8;
        self.state.rotate_left(rot);
        // 3. Simplified MDS-like Diffusion
        for i in (0..STATE_SIZE).step_by(4) {
            let t = self.state[i] ^ self.state[i + 1] ^ self.state[i + 2] ^ self.state[i + 3];
            self.state[i]     ^= t ^ ((t << 1) & 0xFF) ^ ((t >> 7) & 0x01);
            self.state[i + 1] ^= t;
            self.state[i + 2] ^= t;
            self.state[i + 3] ^= t;
        }
        // 4. Add Round Constant (Symmetry breaking)
        let rc = ((round_idx + 1) as u8).wrapping_mul(0x9E);
        self.state[0] ^= rc;
        self.state[1] ^= (rc << 1) | (rc >> 7);
    }

    /// Internal block absorb (must be exactly RATE bytes)
    fn absorb_block(&mut self, block: &[u8]) {
        debug_assert_eq!(block.len(), RATE);
        for i in 0..RATE {
            self.state[i] ^= block[i];
        }
        for r in 0..ROUNDS {
            self.permutation(r);
        }
    }

    /// Applies padding & finalizes internal state
    fn finalize_internal(&mut self) {
        let mut padded = self.buffer.clone();
        padded.push(0x01); // Start padding
        while padded.len() % RATE != 0 {
            padded.push(0x00);
        }
        padded[padded.len() - 1] |= 0x80; // End padding marker

        for chunk in padded.chunks(RATE) {
            self.absorb_block(chunk);
        }
        self.buffer.clear();
    }
    /// 🔑 Keyed Sponge MAC Generation (Native Mode)
    /// Domain Separator: 0x01 ensures Key/Message boundary
    pub fn hash_keyed(key: &[u8], message: &[u8]) -> [u8; 57] {
        let mut h = Self::new();
        h.absorb(key);
        h.absorb(&[0x01]); // Domain separation
        h.absorb(message);
        h.squeeze()
    }

    /// Extracts 456-bit (57-byte) output
    pub fn squeeze(&mut self) -> [u8; 57] {
        let mut out = [0u8; 57];
        let mut offset = 0;
        while offset < 57 {
            let len = std::cmp::min(RATE, 57 - offset);
            out[offset..offset + len].copy_from_slice(&self.state[..len]);
            offset += len;
            if offset < 57 {
                for r in 0..ROUNDS {
                    self.permutation(r);
                }
            }
        }
        out
    }
}

// ─────────────────────────────────────────────────────────────
// digest CRATE TRAIT IMPLEMENTATIONS
// ─────────────────────────────────────────────────────────────
impl OutputSizeUser for Hash456 {
    type OutputSize = U57;
}

impl Update for Hash456 {
    fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
        while self.buffer.len() >= RATE {
            let block: [u8; RATE] = self.buffer.drain(..RATE).collect::<Vec<_>>().try_into().unwrap();
            self.absorb_block(&block);
        }
    }
}

impl Reset for Hash456 {
    fn reset(&mut self) {
        self.state = [0u8; STATE_SIZE];
        self.buffer.clear();
    }}

impl FixedOutput for Hash456 {
    fn finalize_into(mut self, out: &mut digest::Output<Self>) {
        self.finalize_internal();
        let result = self.squeeze();
        out.copy_from_slice(&result);
    }
}

// ─────────────────────────────────────────────────────────────
// TESTS
// ─────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use digest::Digest;

    #[test]
    fn test_standard_hash() {
        let mut hasher = Hash456::new();
        hasher.update(b"Hello Ethical Hacker");
        let res = hasher.finalize_fixed();
        assert_eq!(res.len(), 57);
        println!("Standard: {}", hex::encode(res));
    }

    #[test]
    fn test_keyed_mac() {
        let tag = Hash456::hash_keyed(b"my_secret_key_32_bytes_long!!", b"secure_message");
        assert_eq!(tag.len(), 57);
        println!("Keyed MAC: {}", hex::encode(tag));
    }

    #[test]
    fn test_deterministic() {
        let h1 = Hash456::digest(b"Cisco Certified");
        let h2 = Hash456::digest(b"Cisco Certified");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_avalanche() {
        let h1 = Hash456::digest(b"Test1");
        let h2 = Hash456::digest(b"Test2");
        let mut diff_bits = 0u32;
        for i in 0..57 {
            diff_bits += (h1[i] ^ h2[i]).count_ones();
        }
        println!("Avalanche: {} bits changed (ideal ~228)", diff_bits);        assert!(diff_bits > 100, "Avalanche effect too weak");
    }
        }
