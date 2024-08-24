/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/mash-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
mod test;

/// Computes a 32-bit Murmur3 hash of the given data.
///
/// Murmur3 is a fast, non-cryptographic hash function suitable for hash-based
/// data structures. This implementation processes data in 4-byte chunks and
/// handles any remaining bytes. The hash computation is initialized with the
/// provided seed value.
///
/// # Parameters
///
/// - `data`: A slice of bytes representing the input data to hash.
/// - `seed`: An initial seed value for the hash computation.
///
/// # Returns
///
/// A `u32` value representing the 32-bit hash of the input data.
///
/// # Example
///
/// ```
/// use mash_rs::murmur3_32;
///
/// let data = b"hello world";
/// let seed = 42; // Arbitrary seed value
/// let hash = murmur3_32(data, seed);
/// println!("Hash value: {:#x}", hash);
/// ```
///
/// # Notes
///
/// - This function is intended for use in hash-based data structures and does
///   not provide cryptographic security.
pub fn murmur3_32(data: &[u8], seed: u32) -> u32 {
    let mut h1 = seed;
    let c1 = 0xcc9e2d51;
    let c2 = 0x1b873593;

    let len = data.len();
    let num_blocks = len / 4;

    for i in 0..num_blocks {
        let start = i * 4;
        let k1 = u32::from_le_bytes(data[start..start + 4].try_into().unwrap());

        let k1 = {
            let mut k1 = k1;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            k1
        };

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
    }

    // Handle the remaining bytes
    let tail = &data[num_blocks * 4..];
    let mut k1 = 0u32;

    match tail.len() {
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        _ => (),
    }

    // Finalization
    h1 ^= len as u32;
    h1 = h1 ^ (h1 >> 16);
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 = h1 ^ (h1 >> 13);
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 = h1 ^ (h1 >> 16);

    h1
}
