# DES.rs

Rust implementation of the Data Encryption Standard (DES).

> [!NOTE]
> I previously implemented the DES block encryption function in Haskell, which you can find in my repository [des.hs](https://github.com/filipondios/des.hs). Unlike this Rust implementation, which also includes deciphering, the Haskell version only supports encryption.

> [!WARNING]  
> This implementation is just made for fun so it should not be used in production.
> Its limited to operating with 64-bit blocks of input and corresponding 
> encrypted 64-bit blocks of output instead of streams of bytes. This implementation
> also does not have a cryptographically secure key generation algorithm.

## Usage

The usage of this algorithm is dead simple, there are only two possible available 
functions for the user: `des::encrypt_block` and `des::decrypt_block`. You have
already an example at `src/main.rs` similar to this code below:

```rust
let block  = 0x0123456789ABCDEF;
let key    = 0x133457799BBCDFF1;
let cipher = des::encrypt_block(block,  key);
let plain  = des::decrypt_block(cipher, key);
```

Oviously both functions expect two 64-bit blocks (represented as two 64-bit
unsigned integers), one for the block to be encrypted or decrypted and the key.
The output to the previous code must be the following:

```
Encrypting block 0123456789abcdef using key 133457799bbcdff1 produces 85e813540f0ab405.
Decrypting block 85e813540f0ab405 using key 133457799bbcdff1 produces 0123456789abcdef.
```

## Triple DES

It is not implemented in this repository but once you have the DES encryption and 
decryption function available, you can easily define the Triple DES encryption and 
decryption functions. In this case, it would be the following functions:

```rust
pub fn encrypt_block_3des(block: u64, key1: u64, key2: u64, key3: u64) -> u64 {
    // Encrypt-Decrypt-Encrypt (EDE) mode
    let step1 = des::encrypt_block(block, key1);
    let step2 = des::decrypt_block(step1, key2);
    encrypt_block(step2, key3)
}

pub fn decrypt_block_3des(block: u64, key1: u64, key2: u64, key3: u64) -> u64 {
    // Decrypt-Encrypt-Decrypt (DED) mode - inverse of EDE
    let step1 = des::decrypt_block(block, key3);
    let step2 = des::encrypt_block(step1, key2);
    decrypt_block(step2, key1)
}
```

## References

- [DES by NIST, FIPS PUB 46-3](https://csrc.nist.gov/files/pubs/fips/46-3/final/docs/fips46-3.pdf)
- [The DES Algorithm Illustrated](https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm)
