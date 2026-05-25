def gf_mul(a, b):
    p = 0
    for _ in range(8):
        if b & 1: p ^= a
        hi_bit_set = a & 0x80
        a = (a << 1) & 0xFF
        if hi_bit_set: a ^= 0x1B # AES Polynomial x^8 + x^4 + x^3 + x + 1
        b >>= 1
    return p

def gen_sbox():
    sbox = [0] * 256
    for i in range(256):
        if i == 0:
            sbox[i] = 0x63 # Affine transform of 0
        else:
            # Multiplicative Inverse
            inv = 1
            for j in range(1, 256):
                if gf_mul(i, j) == 1:
                    inv = j
                    break
            # Affine Transform: y = M*x + b
            # Simplified for demo (Standard AES affine map is complex matrix mult)
            x = inv
            y = x ^ ((x << 1) | (x >> 7)) ^ ((x << 2) | (x >> 6)) ^ ((x << 3) | (x >> 5)) ^ ((x << 4) | (x >> 4)) ^ 0x63
            sbox[i] = y & 0xFF
    return sbox

def check_diff_uniformity(sbox):
    max_diff = 0
    for a in range(1, 256):
        diffs = [0] * 256
        for x in range(256):
            b = sbox[x] ^ sbox[x ^ a]
            diffs[b] += 1
        max_diff = max(max_diff, max(diffs))
    return max_diff

sbox = gen_sbox()
print(f"S-Box Generated. Max Differential Uniformity: {check_diff_uniformity(sbox)}")
# Output should be 4 (Excellent for 8-bit)
print(f"SBOX_CONST: [{', '.join(hex(x) for x in sbox)}]")
