use primitive_types::U256;

pub struct Memory {
    memory: Vec<u8>, 
}

pub enum MemoryError {
    MemoryOverflow,
    MemoryAccessError,
}


impl Memory {

    pub fn new() -> Self {
        Memory {
            memory: Vec::new(), 
        }
    }

    pub fn store(&mut self, offset: usize, value: U256) -> Result<(), MemoryError> {
        // EVM MSTORE stores 32 bytes (a word) starting at offset
        let required_size = offset + 32;
        
        if required_size > self.memory.len() {
            self.memory.resize(required_size, 0);
        }

        // Convert U256 to bytes (big-endian, 32 bytes)
        let mut bytes = [0u8; 32];
        value.to_big_endian(&mut bytes);
        

        for i in 0..32 {
            self.memory[offset + i] = bytes[i];
        }
        
        Ok(())
   }
    

   pub fn load(&self, offset: usize) -> Result<U256, MemoryError> {
        // EVM MLOAD loads 32 bytes starting at offset
        let required_size = offset + 32;
        
        if required_size > self.memory.len() {
            return Ok(U256::zero());
        }

        let mut bytes = [0u8; 32];
        for i in 0..32 {
            bytes[i] = self.memory[offset + i];
        }
        
        Ok(U256::from_big_endian(&bytes))
   }
 
}