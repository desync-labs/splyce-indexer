
use super::constants::DISCRIMINATOR_LENGTH;
use sha2::{Sha256, Digest};

pub fn read_descriminator(data: &[u8]) -> &[u8] {
    return &data[0..DISCRIMINATOR_LENGTH];
}

pub fn get_descriminator(event_name:&str) -> [u8; DISCRIMINATOR_LENGTH] {
    let mut hasher = Sha256::new();
        
    // Update the hasher with the struct name
    let struct_name = "event:".to_string() + &event_name;
    hasher.update(struct_name.as_bytes());
    
    // Finalize the hash
    let result = hasher.finalize();
    
    // Extract the first 8 bytes of the hash
    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);
    discriminator
}