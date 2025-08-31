# DES

Rust implementation of the Data Encryption Standard (DES) and Triple Data Encryption Standard (TDES).

> [!NOTE]
> I previously implemented the DES block encryption function in Haskell, which you can find in my
> repository [des.hs](https://github.com/filipondios/des.hs). Unlike this Rust implementation,
> which also includes deciphering, the Haskell version only supports encryption.

> [!WARNING]  
> This implementation is just made for fun so it should not be used in production.
> Its limited to operating with 64-bit blocks of input and corresponding 
> encrypted 64-bit blocks of output instead of streams of bytes.

## Usage

The usage of this algorithm is dead simple, there are only two available DES functions for the
user: `des::encrypt_block` and `des::decrypt_block` and two TDES functions: `tdes::encrypt_block` 
and `tdes::decrypt_block`. You have already an example at `src/main.rs` similar to this code below:

```rust
let block = 0x0123456789ABCDEF;
let keys = [0x0123456789ABCDEF, 0xFEDCBA9876543210, 0x89ABCDEF01234567];

let cipher = des::encrypt_block(block,  keys[0]);
let plain  = des::decrypt_block(cipher, keys[0]); 

let cipher = tdes::encrypt_block(block,  keys[0], keys[1], keys[2]);
let plain  = tdes::decrypt_block(cipher, keys[0], keys[1], keys[2]);
```

Oviously both functions expect two 64-bit blocks (represented as two 64-bit
unsigned integers), one for the block to be encrypted or decrypted and the key.
The output to the previous code must be the following:

```
DES Encryption/Decryption
Encrypting block 0123456789abcdef using key 0123456789abcdef produces 56cc09e7cfdc4cef.
Decrypting block 56cc09e7cfdc4cef using key 0123456789abcdef produces 0123456789abcdef.

TDES Encryption/Decryption
Encrypting block 0123456789abcdef using keys (0123456789abcdef, fedcba9876543210, 89abcdef01234567), produces 691747fd88b6d228.
Decrypting block 691747fd88b6d228 using keys (0123456789abcdef, fedcba9876543210, 89abcdef01234567), produces 0123456789abcdef.
```

## References

- [DES by NIST, FIPS PUB 46-3](https://csrc.nist.gov/files/pubs/fips/46-3/final/docs/fips46-3.pdf)
- [The DES Algorithm Illustrated](https://page.math.tu-berlin.de/~kant/teaching/hess/krypto-ws2006/des.htm)
