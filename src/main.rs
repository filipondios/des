mod tables;
mod bits;
mod des;
mod tdes;

fn main() {
    let block = 0x0123456789ABCDEF;
    let keys = [0x0123456789ABCDEF, 0xFEDCBA9876543210, 0x89ABCDEF01234567];

    let cipher = des::encrypt_block(block,  keys[0]);
    let plain  = des::decrypt_block(cipher, keys[0]); 

    println!("DES Encryption/Decryption");
    println!("Encrypting block {:016x} using key {:016x} produces {:016x}.", block,  keys[0], cipher);
    println!("Decrypting block {:016x} using key {:016x} produces {:016x}.", cipher, keys[0], plain);
       
    let cipher = tdes::encrypt_block(block,  keys[0], keys[1], keys[2]);
    let plain  = tdes::decrypt_block(cipher, keys[0], keys[1], keys[2]);

    println!("\nTDES Encryption/Decryption");
    println!("Encrypting block {:016x} using keys ({:016x}, {:016x}, {:016x}), produces {:016x}.",
         block, keys[0], keys[1], keys[2], cipher);
    println!("Decrypting block {:016x} using keys ({:016x}, {:016x}, {:016x}), produces {:016x}.",
        cipher, keys[0], keys[1], keys[2], plain);
}
