use context::ExecutionContext;
use primitive_types::U256;
use stack::StackError;
use memory::MemoryError;

pub enum InstructionError {
    InvalidOpcode,
    StackError(StackError),
    MemoryError(MemoryError),
    InvalidJump,
}

// Stop and Arithmetic
pub const STOP: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const MUL: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const DIV: u8 = 0x04;
pub const MOD: u8 = 0x06;
pub const EXP: u8 = 0x0a;

// Comparison & Bitwise
pub const LT: u8 = 0x10;
pub const GT: u8 = 0x11;
pub const EQ: u8 = 0x14;
pub const ISZERO: u8 = 0x15;
pub const AND: u8 = 0x16;
pub const OR: u8 = 0x17;
pub const XOR: u8 = 0x18;
pub const NOT: u8 = 0x19;

// SHA3
pub const SHA3: u8 = 0x20;

// Environment Information
pub const ADDRESS: u8 = 0x30;
pub const BALANCE: u8 = 0x31;
pub const ORIGIN: u8 = 0x32;
pub const CALLER: u8 = 0x33;
pub const CALLVALUE: u8 = 0x34;
pub const CALLDATALOAD: u8 = 0x35;
pub const CALLDATASIZE: u8 = 0x36;
pub const CALLDATACOPY: u8 = 0x37;
pub const CODESIZE: u8 = 0x38;
pub const CODECOPY: u8 = 0x39;
pub const GASPRICE: u8 = 0x3a;
pub const EXTCODESIZE: u8 = 0x3b;
pub const EXTCODECOPY: u8 = 0x3c;

// Block Information
pub const BLOCKHASH: u8 = 0x40;
pub const COINBASE: u8 = 0x41;
pub const TIMESTAMP: u8 = 0x42;
pub const NUMBER: u8 = 0x43;
pub const GASLIMIT: u8 = 0x45;

// Storage, Memory and Flow Operations
pub const POP: u8 = 0x50;
pub const MLOAD: u8 = 0x51;
pub const MSTORE: u8 = 0x52;
pub const MSTORE8: u8 = 0x53;
pub const SLOAD: u8 = 0x54;
pub const SSTORE: u8 = 0x55;
pub const JUMP: u8 = 0x56;
pub const JUMPI: u8 = 0x57;
pub const PC: u8 = 0x58;
pub const MSIZE: u8 = 0x59;
pub const GAS: u8 = 0x5a;
pub const JUMPDEST: u8 = 0x5b;

// Push Operations (0x60-0x7f)
pub const PUSH1: u8 = 0x60;
pub const PUSH2: u8 = 0x61;
pub const PUSH3: u8 = 0x62;
pub const PUSH4: u8 = 0x63;
pub const PUSH5: u8 = 0x64;
pub const PUSH6: u8 = 0x65;
pub const PUSH7: u8 = 0x66;
pub const PUSH8: u8 = 0x67;
pub const PUSH32: u8 = 0x7f;

// Dup Operations (0x80-0x8f)
pub const DUP1: u8 = 0x80;
pub const DUP2: u8 = 0x81;
pub const DUP3: u8 = 0x82;
pub const DUP4: u8 = 0x83;

// Swap Operations (0x90-0x9f)
pub const SWAP1: u8 = 0x90;
pub const SWAP2: u8 = 0x91;
pub const SWAP3: u8 = 0x92;
pub const SWAP4: u8 = 0x93;

// Logging
pub const LOG0: u8 = 0xa0;
pub const LOG1: u8 = 0xa1;
pub const LOG2: u8 = 0xa2;
pub const LOG3: u8 = 0xa3;
pub const LOG4: u8 = 0xa4;

// System Operations
pub const CREATE: u8 = 0xf0;
pub const CALL: u8 = 0xf1;
pub const RETURN: u8 = 0xf3;
pub const DELEGATECALL: u8 = 0xf4;
pub const REVERT: u8 = 0xfd;
pub const SELFDESTRUCT: u8 = 0xff;

pub fn execute_opcode(opcode: u8, ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    match opcode {
        // Stop
        STOP => handleStop(ctx),
        
        // Arithmetic
        ADD => handleAdd(ctx),
        MUL => handleMul(ctx),
        SUB => handleSub(ctx),
        DIV => handleDiv(ctx),
        MOD => handleMod(ctx),
        EXP => handleExp(ctx),
        
        // Comparison
        LT => handleLt(ctx),
        GT => handleGt(ctx),
        EQ => handleEq(ctx),
        ISZERO => handleIsZero(ctx),
        
        // Bitwise
        AND => handleAnd(ctx),
        OR => handleOr(ctx),
        XOR => handleXor(ctx),
        NOT => handleNot(ctx),
        
        // Stack
        POP => handlePop(ctx),
        DUP1 => handleDup1(ctx),
        DUP2 => handleDup2(ctx),
        DUP3 => handleDup3(ctx),
        DUP4 => handleDup4(ctx),
        SWAP1 => handleSwap1(ctx),
        SWAP2 => handleSwap2(ctx),
        SWAP3 => handleSwap3(ctx),
        SWAP4 => handleSwap4(ctx),
        
        // Memory
        MLOAD => handleMload(ctx),
        MSTORE => handleMstore(ctx),
        MSTORE8 => handleMstore8(ctx),
        MSIZE => handleMsize(ctx),
        
        // Control Flow
        JUMP => handleJump(ctx),
        JUMPI => handleJumpi(ctx),
        JUMPDEST => handleJumpdest(ctx),
        PC => handlePc(ctx),
        
        // Push Operations
        PUSH1 => handlePush1(ctx),
        PUSH2 => handlePush2(ctx),
        PUSH3 => handlePush3(ctx),
        PUSH4 => handlePush4(ctx),
        PUSH5 => handlePush5(ctx),
        PUSH6 => handlePush6(ctx),
        PUSH7 => handlePush7(ctx),
        PUSH8 => handlePush8(ctx),
        PUSH32 => handlePush32(ctx),
        
        // Environment
        ADDRESS => handleAddress(ctx),
        CALLER => handleCaller(ctx),
        CALLVALUE => handleCallvalue(ctx),
        CALLDATALOAD => handleCalldataload(ctx),
        CALLDATASIZE => handleCalldatasize(ctx),
        CALLDATACOPY => handleCalldatacopy(ctx),
        CODESIZE => handleCodesize(ctx),
        CODECOPY => handleCodecopy(ctx),
        
        // Return
        RETURN => handleReturn(ctx),
        
        _ => Err(InstructionError::InvalidOpcode),
    }
}

// Stop
fn handleStop(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stop();
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Arithmetic Operations
fn handleAdd(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a.overflowing_add(b).0;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleMul(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a.overflowing_mul(b).0;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleSub(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a.overflowing_sub(b).0;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleDiv(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if b.is_zero() { U256::zero() } else { a / b };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleMod(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if b.is_zero() { U256::zero() } else { a % b };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleExp(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let exponent = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let base = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    // U256 doesn't have pow, so we use a simple implementation
    // For large exponents, this could be optimized
    let result = if exponent.is_zero() {
        U256::from(1)
    } else if base.is_zero() {
        U256::zero()
    } else {
        // Simple exponentiation - for production, use a more efficient algorithm
        let mut result = U256::from(1);
        let mut exp = exponent;
        let mut base_pow = base;
        while !exp.is_zero() {
            if exp & U256::from(1) != U256::zero() {
                result = result.overflowing_mul(base_pow).0;
            }
            base_pow = base_pow.overflowing_mul(base_pow).0;
            exp = exp >> 1;
        }
        result
    };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Comparison Operations
fn handleLt(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if a < b { U256::from(1) } else { U256::zero() };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleGt(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if a > b { U256::from(1) } else { U256::zero() };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleEq(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if a == b { U256::from(1) } else { U256::zero() };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleIsZero(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = if a.is_zero() { U256::from(1) } else { U256::zero() };
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Bitwise Operations
fn handleAnd(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a & b;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleOr(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a | b;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleXor(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let b = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = a ^ b;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleNot(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let a = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let result = !a;
    ctx.stack_mut().push(result).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Stack Operations
fn handlePop(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleDup1(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let value = ctx.stack().peek(0).map_err(InstructionError::StackError)?;
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleDup2(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let value = ctx.stack().peek(1).map_err(InstructionError::StackError)?;
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleDup3(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let value = ctx.stack().peek(2).map_err(InstructionError::StackError)?;
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleDup4(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let value = ctx.stack().peek(3).map_err(InstructionError::StackError)?;
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleSwap1(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stack_mut().swap(1).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleSwap2(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stack_mut().swap(2).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleSwap3(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stack_mut().swap(3).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleSwap4(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    ctx.stack_mut().swap(4).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Memory Operations
fn handleMload(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let value = ctx.memory().load(offset.as_usize())
        .map_err(InstructionError::MemoryError)?;
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleMstore(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let value = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    ctx.memory_mut().store(offset.as_usize(), value)
        .map_err(InstructionError::MemoryError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleMstore8(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let value = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    // MSTORE8 stores only the least significant byte
    let byte = (value & U256::from(0xff)).as_u32() as u8;
    ctx.memory_mut().store_byte(offset.as_usize(), byte);
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleMsize(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let size = ctx.memory().size();
    ctx.stack_mut().push(U256::from(size)).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Control Flow
fn handleJump(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let dest = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let dest_usize = dest.as_usize();
    
    // Validate jump destination
    if dest_usize >= ctx.code().len() {
        return Err(InstructionError::InvalidJump);
    }
    
    // Check if destination is JUMPDEST
    if ctx.code()[dest_usize] != JUMPDEST {
        return Err(InstructionError::InvalidJump);
    }
    
    ctx.set_pc(dest_usize);
    Ok(())
}

fn handleJumpi(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let dest = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let condition = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    
    if !condition.is_zero() {
        // Condition is true, perform jump
        let dest_usize = dest.as_usize();
        
        if dest_usize >= ctx.code().len() {
            return Err(InstructionError::InvalidJump);
        }
        
        if ctx.code()[dest_usize] != JUMPDEST {
            return Err(InstructionError::InvalidJump);
        }
        
        ctx.set_pc(dest_usize);
    } else {
        // Condition is false, just advance PC
        ctx.set_pc(ctx.pc() + 1);
    }
    
    Ok(())
}

fn handleJumpdest(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    // JUMPDEST is a no-op, just advance PC
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handlePc(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let pc_value = U256::from(ctx.pc());
    ctx.stack_mut().push(pc_value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Push Operations
fn handlePush1(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let value_byte = ctx.read_code(1)[0];
    let value = U256::from(value_byte);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 2);
    Ok(())
}

fn handlePush2(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(2);
    let value = U256::from_big_endian(&bytes);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 3);
    Ok(())
}

fn handlePush3(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(3);
    let mut padded = vec![0u8; 32];
    padded[29..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 4);
    Ok(())
}

fn handlePush4(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(4);
    let mut padded = vec![0u8; 32];
    padded[28..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 5);
    Ok(())
}

fn handlePush5(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(5);
    let mut padded = vec![0u8; 32];
    padded[27..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 6);
    Ok(())
}

fn handlePush6(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(6);
    let mut padded = vec![0u8; 32];
    padded[26..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 7);
    Ok(())
}

fn handlePush7(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(7);
    let mut padded = vec![0u8; 32];
    padded[25..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 8);
    Ok(())
}

fn handlePush8(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(8);
    let mut padded = vec![0u8; 32];
    padded[24..].copy_from_slice(&bytes);
    let value = U256::from_big_endian(&padded);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 9);
    Ok(())
}

fn handlePush32(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let bytes = ctx.read_code(32);
    let value = U256::from_big_endian(&bytes);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 33);
    Ok(())
}

// Environment Operations
fn handleAddress(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let addr = ctx.contract_address();
    let mut bytes = [0u8; 32];
    bytes[12..].copy_from_slice(addr);
    let value = U256::from_big_endian(&bytes);
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCaller(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    // TODO: CALLER should come from transaction context, not contract address
    // For now, return zero
    ctx.stack_mut().push(U256::zero()).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCallvalue(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    // TODO: CALLVALUE should come from transaction context
    // For now, return zero
    ctx.stack_mut().push(U256::zero()).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCalldataload(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let value = ctx.calldata().load(offset.as_usize());
    ctx.stack_mut().push(value).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCalldatasize(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let size = ctx.calldata().size();
    ctx.stack_mut().push(U256::from(size)).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCalldatacopy(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let mem_offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let calldata_offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let length = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    
    ctx.calldata().copy_to_memory(
        calldata_offset.as_usize(),
        mem_offset.as_usize(),
        length.as_usize(),
        ctx.memory_mut()
    ).map_err(|_| InstructionError::InvalidOpcode)?;
    
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCodesize(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let size = ctx.code().len();
    ctx.stack_mut().push(U256::from(size)).map_err(InstructionError::StackError)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

fn handleCodecopy(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let mem_offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let code_offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let length = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    
    let code_offset_usize = code_offset.as_usize();
    let length_usize = length.as_usize();
    let mem_offset_usize = mem_offset.as_usize();
    
    // Copy code to memory
    let mut bytes = Vec::new();
    for i in 0..length_usize {
        if code_offset_usize + i < ctx.code().len() {
            bytes.push(ctx.code()[code_offset_usize + i]);
        } else {
            bytes.push(0);
        }
    }
    
    ctx.memory_mut().store_bytes(mem_offset_usize, &bytes);
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}

// Return
fn handleReturn(ctx: &mut ExecutionContext) -> Result<(), InstructionError> {
    let offset = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    let length = ctx.stack_mut().pop().map_err(InstructionError::StackError)?;
    
    ctx.set_return_data(offset.as_usize(), length.as_usize())
        .map_err(|_| InstructionError::InvalidOpcode)?;
    ctx.set_pc(ctx.pc() + 1);
    Ok(())
}
