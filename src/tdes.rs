use crate::des; 

pub fn encrypt_block(block: u64, key1: u64, key2: u64, key3: u64) -> u64 {
    // Encrypt-Decrypt-Encrypt (EDE) mode
    let step1 = des::encrypt_block(block, key1);
    let step2 = des::decrypt_block(step1, key2);
    des::encrypt_block(step2, key3)
}

pub fn decrypt_block(block: u64, key1: u64, key2: u64, key3: u64) -> u64 {
    // Decrypt-Encrypt-Decrypt (DED) mode - inverse of EDE
    let step1 = des::decrypt_block(block, key3);
    let step2 = des::encrypt_block(step1, key2);
    des::decrypt_block(step2, key1)
}
