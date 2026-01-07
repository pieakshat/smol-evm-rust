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

    fn read_byte(&self, offset: usize) -> Result<u8, CalldataError> {

        if offset >= self.data.len() {
            return 0; // default value for out-of-bounds access
        }

        Ok(self.data[offset])
    }

    
}