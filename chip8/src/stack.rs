// https://www.kirillvasiltsov.com/writing/how-to-write-a-stack-in-rust/

struct Stack<T> {
  stack: Vec<T>,
}

impl<T> Stack<T> {
  fn new() -> Self {
    Stack { stack : Vec::new() }
  }

  fn pop(&mut self) -> Option<T> { // Stack could be empty, so Option<T> can return None in that case
    self.stack.pop()
  }
  
  fn push(&mut self) {
    self.stack.push()
  }
  
  fn is_empty(&self) -> bool {
    self.stack.is_empty()
  }
  
  fn length(&self) -> usize {
    self.stack.len()
  }
  
  fn peek(&self) -> Option<&T> { // Returns a reference to the object to avoid removing it
    self.stack.last()
  }
}

