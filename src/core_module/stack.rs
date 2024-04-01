use super::utils::errors::ExecutionError;

/// A stack data structure used in EVM to store and manipulate data.
#[derive(Debug)]
pub struct Stack{
    // Ken:The length of each element in the array stack is u8,32bytes.
    pub stack: Vec<[u8;32]>
}

impl Stack{
    // Ken: Create a new stack instance.
    pub fn new() -> Self{
        Self{
            stack: vec![]
        }
    }

    /// Pushes a 32-byte word onto the stack.
    ///
    /// # Arguments
    ///
    /// * `word` - A 32-byte array representing the word to be pushed onto the stack.
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the stack is too deep (i.e., has more than 1024 elements).
    pub fn push(&mut self, word: [u8; 32]) ->Result<(),ExecutionError>{
        // check if stack is too deep!
        if self.stack.len() >= 1024{
            // Return an error
            return  Err(ExecutionError::StackTooDeep);
        }

        Ok(self.stack.push(word))
    }

    /// Pop a word off the stack
    ///
    /// # Errors
    ///
    /// Returns an `ExecutionError` if the stack is empty.
    pub fn pop(&mut self) -> Result<[u8;32],ExecutionError>{
        // check if the stack is empty.
        if self.stack.is_empty(){
            // Return an error
            return Err(ExecutionError::StackTooSmall)
        }
    }






}