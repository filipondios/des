//! The `des` module provides the necessary functions to encrypt and decrypt 
//! 64-bit blocks using the DES algorithm as you naturally would expect. Those
//! functions are the only two public ones in this module: `encrypt_block` and 
//! `decrypt_block`.

use crate::bits;
use crate::tables;

fn f(r: u32, key: u64) -> u32 {
    let block = bits::permute64((r as u64) << 32, &tables::E) ^ key;
    let mut s_mask = 0xFC00000000000000;
    let mut s_result = 0;

    for (iteration, table) in tables::SBOX.iter().enumerate() {
        let b = (s_mask & block) >> (64 - (6 * (iteration + 1)));
        let row = (0b000001 & b) | ((0b100000 & b) >> 4);
        let col = (0b011110 & b) >> 1;

        let s_val = table[row as usize][col as usize] as u32;
        s_result |= s_val << (32 - (4 * (iteration + 1)));
        s_mask >>= 6;
    }
    bits::permute32(s_result, &tables::P)
}

fn key_schedule(c: u32, d: u32, iter: usize) -> (u64, u32, u32) {
    let shifts = tables::SHIFTS[iter];
    let c = bits::rotate28(c, shifts);
    let d = bits::rotate28(d, shifts);
    (bits::permute64(bits::join28s(c, d), &tables::PC2), c, d)
}

pub fn encrypt_block(block: u64, key: u64) -> u64 {
    let (mut l, mut r) = bits::split64(bits::permute64(block, &tables::IP));
    let (mut c, mut d) = bits::split56(bits::permute64(key, &tables::PC1));
    let mut key;

    for round in 0..16 {
        (key, c, d) = key_schedule(c, d, round);

        (l, r) = if round != 15 {
            (r, l ^ f(r, key))
        } else {
            (l ^ f(r, key), r)
        }
    }
    bits::permute64(bits::join32s(l, r), &tables::IP_INV)
}

pub fn decrypt_block(block: u64, key: u64) -> u64 {
    let (mut l, mut r) = bits::split64(bits::permute64(block, &tables::IP));
    let (mut c, mut d) = bits::split56(bits::permute64(key, &tables::PC1));
    let mut keys = [0; 16];
    let mut key;

    for round in 0..16 {
        (key, c, d) = key_schedule(c, d, round);
        keys[round] = key;
    }

    for round in (0..16).rev() {
        (l, r) = if round != 0 {
            (r, l ^ f(r, keys[round]))
        } else {
            (l ^ f(r, keys[round]), r)
        }
    }
    bits::permute64(bits::join32s(l, r), &tables::IP_INV)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f() {
        assert_eq!(f(0xF0AAF0AA,  0x1B02EFFC70720000), 0x234AA9BB);
    }

    #[test]
    fn test_key_schedule() {
        let (mut c, mut d) = (0xF0CCAAF0, 0x556678F0);
        let mut key;

        let cd_rounds: [[u32; 2]; 16] = [
            [0xE19955F0, 0xAACCF1E0],
            [0xC332ABF0, 0x5599E3D0],
            [0x0CCAAFF0, 0x56678F50],
            [0x332ABFC0, 0x599E3D50],
            [0xCCAAFF00, 0x6678F550],
            [0x32ABFC30, 0x99E3D550],
            [0xCAAFF0C0, 0x678F5560],
            [0x2ABFC330, 0x9E3D5590],
            [0x557F8660, 0x3C7AAB30],
            [0x55FE1990, 0xF1EAACC0],
            [0x57F86650, 0xC7AAB330],
            [0x5FE19950, 0x1EAACCF0],
            [0x7F866550, 0x7AAB33C0],
            [0xFE199550, 0xEAACCF10],
            [0xF8665570, 0xAAB33C70],
            [0xF0CCAAF0, 0x556678F0],
        ];

        let keys: [u64; 16] = [
            0x1B02EFFC70720000,
            0x79AED9DBC9E50000,
            0x55FC8A42CF990000,
            0x72ADD6DB351D0000,
            0x7CEC07EB53A80000,
            0x63A53E507B2F0000,
            0xEC84B7F618BC0000,
            0xF78A3AC13BFB0000,
            0xE0DBEBEDE7810000,
            0xB1F347BA464F0000,
            0x215FD3DED3860000,
            0x7571F59467E90000,
            0x97C5D1FABA410000,
            0x5F43B7F2E73A0000,
            0xBF918D3D3F0A0000,
            0xCB3D8B0E17F50000,
        ];
    
        for round in 0..16 {
            (key, c, d) = key_schedule(c, d, round);
            assert_eq!(c, cd_rounds[round][0]);
            assert_eq!(d, cd_rounds[round][1]);
            assert_eq!(key, keys[round]);
        }
    }

    #[test]
    fn test_encrypt_block() {
        let block = 0x0123456789ABCDEF;
        let key = 0x133457799BBCDFF1;
        assert_eq!(encrypt_block(block, key), 0x85E813540F0AB405);

        let block = 0x8787878787878787;
        let key = 0x0E329232EA6D0D73;
        assert_eq!(encrypt_block(block, key), 0x0);
    }

    #[test]
    fn test_decrypt_block() {
        let block = 0x85E813540F0AB405;
        let key = 0x133457799BBCDFF1;
        assert_eq!(decrypt_block(block, key), 0x0123456789ABCDEF);
    }
}
