When is `ExecutionContext` created?

  ExecutionContext is created only when a function/transaction is called, not always.

  Lifecycle


     1 │┌─────────────────────────────────────────┐
     2 ││  Contract deployed on blockchain        │
     3 ││  (bytecode stored, but NOT executing)  │
     4 │└─────────────────────────────────────────┘
     5 │              │
     6 │              │ User calls a function
     7 │              ▼
     8 │┌─────────────────────────────────────────┐
     9 ││  NEW ExecutionContext created           │
    10 ││  - Load contract bytecode → code        │
    11 ││  - Initialize empty stack, memory       │
    12 ││  - Set calldata from function call      │
    13 ││  - pc = 0, stopped = false              │
    14 │└─────────────────────────────────────────┘
    15 │              │
    16 │              │ Execute bytecode
    17 │              ▼
    18 │┌─────────────────────────────────────────┐
    19 ││  ExecutionContext runs                   │
    20 ││  - Read opcodes from code[pc]           │
    21 ││  - Push/pop values on stack              │
    22 ││  - Store/load from memory                │
    23 ││  - Update pc, check if stopped           │
    24 │└─────────────────────────────────────────┘
    25 │              │
    26 │              │ Execution completes
    27 │              ▼
    28 │┌─────────────────────────────────────────┐
    29 ││  ExecutionContext destroyed              │
    30 ││  (or returned as result)                 │
    31 │└─────────────────────────────────────────┘

  ---

  How it works in practice


  Scenario 1: Contract deployed but idle


     1 │// Contract exists on blockchain
     2 │// But NO ExecutionContext exists yet
     3 │// Nothing is executing


  Scenario 2: User calls `transfer(address to, uint256 amount)`


     1 │// 1. Create NEW ExecutionContext
     2 │let mut ctx = ExecutionContext::new();
     3 │
     4 │// 2. Initialize it with contract data
     5 │ctx.code = load_contract_bytecode(); // Load from blockchain
     6 │ctx.calldata = Calldata::new(encoded_function_call); // From transaction
     7 │ctx.pc = 0;
     8 │ctx.stopped = false;
     9 │
    10 │// 3. Execute the bytecode
    11 │while !ctx.stopped {
    12 │    let opcode = ctx.read_code(1)[0];
    13 │    execute_opcode(opcode, &mut ctx);
    14 │    ctx.pc += 1; // Move to next instruction
    15 │}
    16 │
    17 │// 4. ExecutionContext is done (destroyed or result returned)


  Scenario 3: Another function call happens


     1 │// A BRAND NEW ExecutionContext is created
     2 │// Previous one is gone (memory/stack cleared)
     3 │let mut ctx2 = ExecutionContext::new();
     4 │// ... execute again

  ---

  Important points

  1. One ExecutionContext per call/transaction
    • Each function call gets its own context
    • Stack and memory are fresh for each call
  2. Not persistent between calls
    • Memory is cleared after each call
    • Stack is cleared after each call
    • Only Storage persists between calls
  3. Nested calls (internal function calls)
    • Each nested call can create its own context or reuse/modify the current one
    • Depends on your EVM design

  ---

  Example flow


     1 │// Contract deployed
     2 │// No ExecutionContext exists
     3 │
     4 │// User 1 calls transfer()
     5 │let ctx1 = ExecutionContext::new();
     6 │ctx1.code = contract_bytecode;
     7 │ctx1.calldata = Calldata::new(transfer_calldata);
     8 │// Execute ctx1...
     9 │// ctx1 destroyed
    10 │
    11 │// User 2 calls approve() (different function)
    12 │let ctx2 = ExecutionContext::new();  // NEW context!
    13 │ctx2.code = contract_bytecode;        // Same contract
    14 │ctx2.calldata = Calldata::new(approve_calldata); // Different call
    15 │// Execute ctx2...
    16 │// ctx2 destroyed
    17 │
    18 │// User 3 calls transfer() again
    19 │let ctx3 = ExecutionContext::new();  // ANOTHER new context!
    20 │// ctx3 is completely independent of ctx1

  ---

  In your EVM implementation

  You'll likely have something like:

     1 │pub struct EVM {
     2 │    // Contract storage (persistent)
     3 │    storage: Storage,
     4 │    
     5 │    // Maybe keep contract bytecode cached?
     6 │    // But ExecutionContext is created per call
     7 │}
     8 │
     9 │impl EVM {
    10 │    pub fn execute_transaction(&mut self, calldata: Vec<u8>) -> Vec<u8> {
    11 │        // Create NEW ExecutionContext for this transaction
    12 │        let mut ctx = ExecutionContext::new();
    13 │        
    14 │        // Initialize it
    15 │        ctx.code = self.load_contract_code();
    16 │        ctx.calldata = Calldata::new(calldata);
    17 │        
    18 │        // Execute
    19 │        self.run_execution(&mut ctx);
    20 │        
    21 │        // Return result
    22 │        ctx.return_data
    23 │        // ctx is destroyed here
    24 │    }
    25 │}

  ---

  Summary

  • ExecutionContext is created when a function/transaction is called
  • It is not always present
  • One context per call
  • Context is destroyed after execution completes
  • Each call gets a fresh stack and memory

  Think of it like a function call in programming: the function's local variables exist only while the function is executing, then they're gone.
  ExecutionContext is similar — it exists only during execution of a contract call.