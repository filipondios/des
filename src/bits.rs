//! The `bits` module provides an abstraction layer for bit-level operations  
//! on 64-bit, 58-bit, and 32-bit blocks used throughout the DES algorithm.  
//!
//! Note: In this implementation, the most significant bit of a block is  
//! considered the first bit (index 0).  

use crate::tables;

fn test_bit64(block: u64, i: u32) -> bool {
    (block & (0x8000000000000000 >> i)) != 0
}

fn set_bit64(block: u64, i: u32) -> u64 {
    block | (0x8000000000000000 >> i)
}

fn test_bit32(block: u32, bit: u32) -> bool {
    (block & (0x80000000 >> bit)) != 0
}

fn set_bit32(block: u32, bit: u32) -> u32 {
    block | (0x80000000 >> bit)
}

pub fn split64(block: u64) -> (u32, u32) {
    let l: u64 = 0xFFFFFFFF00000000 & block;
    let r: u64 = 0x00000000FFFFFFFF & block; 
    ((l >> 32) as u32, r as u32)
}

pub fn join32s(left: u32, right: u32) -> u64 {
    ((left as u64) << 32) | (right as u64)
}

pub fn split56(block: u64) -> (u32, u32) {
    let c: u64 = block & 0xFFFFFFF000000000;
    let d: u64 = block & 0xFFFFFFF00;
    ((c >> 32) as u32, (d >> 4) as u32)
} 

pub fn join28s(c: u32, d: u32) -> u64 {
    ((c as u64) << 32) | ((d as u64) << 4)
}

pub fn rotate28(block: u32, shifts: tables::Shift) -> u32 {
    let first_bits = match shifts {
        tables::Shift::One => (block & 0x80000000) >> 27,
        tables::Shift::Two => (block & 0xC0000000) >> 26,
    };
    (block.overflowing_shl(shifts as u32).0) | first_bits
}

pub fn permute64(block: u64, table: &[u32]) -> u64 {
    table.iter().enumerate().fold(0, |result, (bit, &index)| {
        if test_bit64(block, index) {
            set_bit64(result, bit as u32)
        } else {
            result
        }
    })
}

pub fn permute32(block: u32, table: &[u32]) -> u32 {
    table.iter().enumerate().fold(0, |result, (bit, &index)| {
        if test_bit32(block, index) {
            set_bit32(result, bit as u32)
        } else {
            result
        }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_bit64() {
        assert!(!test_bit64(0x123456789ABCDEF, 22));
        assert!(test_bit64(0x123456789ABCDEF, 62));
        assert!(!test_bit64(0x123456789ABCDEF, 24));
        assert!(test_bit64(0x133457799BBCDFF1, 3));
        assert!(!test_bit64(0x133457799BBCDFF1, 20));
        assert!(test_bit64(0x133457799BBCDFF1, 38));
    }

    #[test]
    fn test_set_bit64() {
        assert_eq!(set_bit64(0x2D24D04CAD8B8083, 27), 0x2D24D05CAD8B8083);
        assert_eq!(set_bit64(0x2D24D05CAD8B8083, 43), 0x2D24D05CAD9B8083);
        assert_eq!(set_bit64(0x2D24D05CAD9B8083, 0),  0xAD24D05CAD9B8083); 
    }
            
    #[test]
    fn test_test_bit32() {
        assert!(!test_bit32(0xF0CCAAF2, 17));
        assert!(test_bit32(0xF0CCAAF2, 26));
        assert!(!test_bit32(0xF0CCAAF2, 31));
        assert!(test_bit32(0x6678F55A, 16));
        assert!(test_bit32(0x6678F55A, 5));
        assert!(!test_bit32(0x6678F55A, 26));
    }

    #[test]
    fn test_set_bit32() {
        assert_eq!(set_bit32(0x4C00CCFE, 0), 0xCC00CCFE);
        assert_eq!(set_bit32(0xCC00CCFE, 31), 0xCC00CCFF);   
        assert_eq!(set_bit32(0xCC00CCFF, 9), 0xCC40CCFF);
    }

    #[test]
    fn test_split64() {
        let (l, r) = split64(0x420F638744EAD0FD);
        assert_eq!(l, 0x420F6387);
        assert_eq!(r, 0x44EAD0FD);

        let (l, r) = split64(0xEE2CE164A50BAC4B);
        assert_eq!(l, 0xEE2CE164);
        assert_eq!(r, 0xA50BAC4B);
    }

    #[test]
    fn test_join32s() {
        let l = 0x420F6387;
        let r = 0x44EAD0FD;
        assert_eq!(join32s(l, r), 0x420F638744EAD0FD);

        let l = 0xEE2CE164;
        let r = 0xA50BAC4B;
        assert_eq!(join32s(l, r), 0xEE2CE164A50BAC4B);
    }

    #[test]
    fn test_split56() {
        let (c, d) = split56(0xF0CCAAF556678F00);
        assert_eq!(c, 0xF0CCAAF0);
        assert_eq!(d, 0x556678F0);
    }

    #[test]
    fn test_join28s() {
        let c = 0xF0CCAAF0;
        let d = 0x556678F0;
        assert_eq!(join28s(c, d), 0xF0CCAAF556678F00);
    }

    #[test]
    fn test_rotate28() {
        assert_eq!(rotate28(0x59D2C690, tables::Shift::Two), 0x674B1A50);
        assert_eq!(rotate28(0xA72D3860, tables::Shift::One), 0x4E5A70D0);
        assert_eq!(rotate28(0xCAB1E520, tables::Shift::Two), 0x2AC794B0);
        assert_eq!(rotate28(0x374E93B0, tables::Shift::Two), 0xDD3A4EC0);
        assert_eq!(rotate28(0x374E93B0, tables::Shift::One), 0x6E9D2760);
    }

    #[test]
    fn test_permute64() {
        assert_eq!(permute64(0x123456789ABCDEF, &tables::IP), 0xCC00CCFFF0AAF0AA);
        assert_eq!(permute64(0xA4CD99543423234, &tables::IP_INV), 0x85E813540F0AB405);
        assert_eq!(permute64(0x133457799BBCDFF1, &tables::PC1), 0xF0CCAAF556678F00);
        assert_eq!(permute64(0xE19955FAACCF1E00, &tables::PC2), 0x1B02EFFC70720000);
        assert_eq!(permute64(0xF0AAF0AA00000000, &tables::E), 0x7A15557A15550000);
    }

    #[test]
    fn test_permute32() {
        assert_eq!(permute32(0x5C82B597, &tables::P), 0x234AA9BB);
    }
}
