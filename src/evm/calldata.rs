use memory::Memory;
use stack::Stack; 
use primitive_types::U256;


pub struct Calldata {
    data: Vec<u8>,
}

pub enum CalldataError {
    InvalidCalldataAccess, 
}

impl Calldata {

    pub fn new(data: Vec<u8>) -> Self {
        Calldata {
            data: data
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn load(&self, offset: usize) -> U256 {
        // CALLDATALOAD loads 32 bytes starting at offset
        let mut bytes = [0u8; 32];
        for i in 0..32 {
            if offset + i < self.data.len() {
                bytes[i] = self.data[offset + i];
            } else {
                bytes[i] = 0;
            }
        }
        U256::from_big_endian(&bytes)
    }

    pub fn copy_to_memory(&self, calldata_offset: usize, memory_offset: usize, length: usize, memory: &mut memory::Memory) -> Result<(), CalldataError> {
        // Copy calldata to memory
        let mut bytes = Vec::new();
        for i in 0..length {
            let byte = if calldata_offset + i < self.data.len() {
                self.data[calldata_offset + i]
            } else {
                0
            };
            bytes.push(byte);
        }
        memory.store_bytes(memory_offset, &bytes);
        Ok(())
    }

    fn read_byte(&self, offset: usize) -> Result<u8, CalldataError> {
        if offset >= self.data.len() {
            return Ok(0); // default value for out-of-bounds access
        }
        Ok(self.data[offset])
    }
}