// https://www.kirillvasiltsov.com/writing/how-to-write-a-stack-in-rust/

pub mod stack {
  pub struct Stack<T> {
    stack: Vec<T>,
  }
  
  impl<T> Stack<T> {
    pub fn new() -> Self {
      Stack { stack : Vec::new() }
    }
  
    pub fn pop(&mut self) -> Option<T> { // Stack could be empty, so Option<T> can return None in that case
      self.stack.pop()
    }
    
    pub fn push(&mut self, item: T) {
      self.stack.push(item)
    }
    
    pub fn is_empty(&self) -> bool {
      self.stack.is_empty()
    }
    
    pub fn length(&self) -> usize {
      self.stack.len()
    }
    
    pub fn peek(&self) -> Option<&T> { // Returns a reference to the object to avoid removing it
      self.stack.last()
    }
  }
}



