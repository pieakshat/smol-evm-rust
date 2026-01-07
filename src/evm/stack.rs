use primitive_types::U256; 
use constants::MAX_DEPTH;

pub enum StackError {
    StackOverflow,
    StackUnderflow, 
    IndexError, 
    InvalidStackItem
}

pub struct Stack {
    data: Vec<U256>, 
    max_depth: usize, 
}

impl Stack {

    pub fn new() -> Self {
        Stack {
            data: Vec::new(), 
            max_depth: MAX_DEPTH,
        }
    }

    pub fn push(&mut self, value: U256) -> Result<(), StackError> {

        if self.data.len() >= self.max_depth {
            return Err(StackError::StackOverflow); 
        }
        self.data.push(value); 
        Ok(())
    }

    pub fn pop(&mut self) -> Result<U256, StackError> {
        if self.data.is_empty() {
            return Err(StackError::StackUnderflow);
        }

        Ok(self.data.pop().unwrap()) 
    }

    pub fn peek(&self, index: usize) -> Result<U256, StackError> {
        if index >= self.data.len() {
            return Err(StackError::IndexError);
        }

        Ok(self.data[self.data.len() - 1 - index])
    }

    pub fn swap(&mut self, n: usize) -> Result<(), StackError> {
        if n + 1 > self.data.len() {
            return Err(StackError::StackOverflow); 
        }

        self.data.swap(self.data.len() - 1, self.data.len() - 1 - n);
        Ok(())
    }

}