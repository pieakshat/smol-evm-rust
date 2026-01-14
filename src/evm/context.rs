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

    pub fn stack_mut(&mut self) -> &mut Stack {
        &mut self.stack
    }

    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    pub fn memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc += 1; 
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
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

    pub fn code(&self) -> &Vec<u8> {
        &self.code
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn stop(&mut self) {
        self.stopped = true; 
    }

    pub fn set_return_data(&mut self, offset: usize, length: usize) -> Result<()> {
        self.stopped = true; 
        self.return_data = self.memory.load_range(offset, length);
        Ok(())
    }

    pub fn return_data(&self) -> &Vec<u8> {
        &self.return_data
    }

    pub fn contract_address(&self) -> &Address {
        &self.contractAddress
    }

}
