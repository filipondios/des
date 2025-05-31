mod tables;
mod bits;
mod des;

fn main() {
    // Example (from des::tests)
    let block = 0x0123456789ABCDEF;
    let key = 0x133457799BBCDFF1;
    let cipher = des::encrypt_block(block, key);
    let plain  = des::decrypt_block(cipher, key); 

    println!("Encrypting block {:016x} using key {:016x} produces {:016x}.",
        block, key, cipher);
    println!("Decrypting block {:016x} using key {:016x} produces {:016x}.",
        cipher, key, plain);
}
