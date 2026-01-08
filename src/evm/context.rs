use stack::Stack;
use memory::Memory;
use calldata::Calldata;
use primitive_types::U256; 
use storage::Storage;

pub type Address = [u8; 20];

pub struct ExecutionContext {
    code: Vec<u8>, 
    stack: Stack, 
    memory: Memory, 
    calldata: Calldata, 
    contractAddress: Address,
    pc: usize, 
    stopped: bool, 
    return_data: Vec<u8>,
}

impl ExecutionContext {

    pub fn new(contractAddress: Address, code: Vec<u8>, calldata: Vec<u8>) -> Self {
        ExecutionContext {
            code,  
            stack: Stack::new(), 
            memory: Memory::new(), 
            calldata: Calldata::new(calldata),  
            contractAddress, 
            pc: 0, 
            stopped: false, 
            return_data: Vec::new() 
        }
    }

    pub fn set_return_data(&mut self, offset: usize, length: usize) -> Result<()> {
        self.stopped = true; 
        self.return_data = self.memory.load_range(offset, length);
        Ok(())
    }

    pub fn stop(&mut self) -> Result<()> {
        self.stopped = true; 
    }

    pub fn set_pc(&mut self, pc: usize) -> Result<()> {
        self.pc = pc; 
        Ok(())
    }

    pub fn read_code(&self, num_bytes: usize) -> Vec<u8> {
        let mut bytes = Vec::new(); 
        for i in 0..num_bytes {
            if self.pc + i < self.code.len() {
                bytes.push(self.code[self.pc + i]); 
            } else {
                bytes.push(0); 
            }
        }
        bytes
    }

}
