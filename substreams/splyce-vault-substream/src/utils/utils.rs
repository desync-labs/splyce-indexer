
use super::constants::DISCRIMINATOR_LENGTH;

pub fn read_descriptor(data: &[u8]) -> &[u8] {
    return &data[0..DISCRIMINATOR_LENGTH];
}