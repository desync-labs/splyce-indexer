//DEPRECATED
pub const DISCRIMINATOR:usize  = 8;

pub const DISCRIMINATOR_LENGTH:usize  = 8;

// pub const VAULT_INIT_DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

pub enum InstructionType {
    VaultInit,
}

// Implement a method or function to get the associated array for each enum variant
//TODO: How not to hardcode this vault init descrminator
impl InstructionType {
    pub fn value(&self) -> [u8; 8] {
        match self {
            InstructionType::VaultInit => [175, 175, 109, 31, 13, 152, 155, 237],
        }
    }

    // Method to compare a byte slice with the enum variant's value
    pub fn matches(&self, data: &[u8]) -> bool {
        &data == &self.value()
    }
}