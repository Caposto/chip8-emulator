mod stack;
use stack::stack::Stack;

fn main() {
  let mut stack = Stack::new();
  stack.push(1);
  stack.push(2);

  println!("Stack length: {}", stack.length());
  if let Some(top) = stack.peek() {
      println!("Top of stack: {}", top);
  }
}