use stack::Stack;
use memory::Memory;
use calldata::Calldata;
use primitive_types::U256; 


struct ExecutionContext {
    code: Vec<u8>, 
    stack: Stack, 
    memory: Memory, 
    calldata: Calldata, 
    pc: usize, 
    stopped: bool, 
    return_data: Vec<u8>,
}

impl ExecutionContext {

}
