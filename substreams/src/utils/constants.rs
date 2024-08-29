//DEPRECATED
pub const DISCRIMINATOR:usize  = 8;

pub const DISCRIMINATOR_LENGTH:usize  = 8;

pub enum VaultInstructions {
    Initialize,
}

// Implement a method or function to get the associated array for each enum variant
//TODO: How not to hardcode this vault init descrminator
impl VaultInstructions {
    pub fn value(&self) -> [u8; 8] {
        match self {
            VaultInstructions::Initialize => [175, 175, 109, 31, 13, 152, 155, 237],
        }
    }

    // Method to compare a byte slice with the enum variant's value
    pub fn matches(&self, data: &[u8]) -> bool {
        &data == &self.value()
    }
}

pub enum VaultStrategyInstructions {
    Initialize,
}

// Implement a method or function to get the associated array for each enum variant
//TODO: How not to hardcode this vault init descrminator
impl VaultStrategyInstructions {
    pub fn value(&self) -> [u8; 8] {
        match self {
            VaultStrategyInstructions::Initialize => [175, 175, 109, 31, 13, 152, 155, 237],
        }
    }

    // Method to compare a byte slice with the enum variant's value
    pub fn matches(&self, data: &[u8]) -> bool {
        &data == &self.value()
    }
}