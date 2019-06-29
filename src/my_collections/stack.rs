/**
  _____________________________________
  |  0  |  1  |  2  | nil | nil | nil |
  -------------------------------------
                    ^
                    |
                 head(3)
*/
pub struct Stack<T> {
  head: usize,
  array: Vec<T>,
}

impl<T> Stack<T> {
  pub fn capacity(&self) -> usize {
    self.array.capacity()
  }
}

impl<T> Stack<T> {
  pub fn with_capacity(capacity: usize) -> Self {
    Stack {
      head: 0,
      array: Vec::with_capacity(capacity),
    }
  }
}

impl<T> Stack<T> {
  pub fn push(&mut self, item: T) {
    if self.is_full() {
      panic!("The stack is full");
    }
    self.array.push(item);
    self.head = self.head + 1;
  }

  pub fn pop(&mut self) -> Option<T> {
    let result = self.array.pop();
    if result.is_some() {
      self.head = self.head - 1;
    }
    result
  }

  pub fn peek(&self) -> Option<&T> {
    if self.is_empty() {
      None
    } else {
      self.array.get(self.head - 1)
    }
  }
}

impl <T> Stack<T> {
  pub fn is_empty(&self) -> bool {
    self.head == 0
  }

  pub fn is_full(&self) -> bool {
    self.head == self.capacity()
  }
}

#[cfg(test)]
mod tests {
  use super::Stack;

  #[test]
  fn capacity() {
    let stack: Stack<i32> = super::Stack::with_capacity(4);
    assert_eq!(stack.capacity(), 4);
  }

  #[test]
  fn const_capacity_after_push() {
    let mut stack: Stack<i32> = super::Stack::with_capacity(4);
    stack.push(3);
    stack.push(2);
    assert_eq!(stack.capacity(), 4);
  }

  #[test]
  fn const_capacity_after_pop() {
    let mut stack: Stack<i32> = super::Stack::with_capacity(4);
    stack.pop();
    assert_eq!(stack.capacity(), 4);
  }

  #[test]
  fn empty_stack_is_empty() {
    let stack: Stack<i32> = Stack::with_capacity(3);
    assert!(stack.is_empty());
  }

  #[test]
  fn full_stack_is_full() {
    let mut stack: Stack<i32> = Stack::with_capacity(1);
    stack.push(2);
    assert!(stack.is_full());
  }

  #[test]
  fn push_and_peek() {
    let mut stack: Stack<i32> = Stack::with_capacity(5);
    assert_eq!(stack.peek(), None);
    stack.push(3);
    assert_eq!(stack.peek(), Some(&3));
  }

  #[test]
  fn push_and_pop() {
    let mut stack:Stack<i32>  = super::Stack::with_capacity(5);
    stack.push(3);
    stack.push(2);
    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn pop_and_push() {
    let mut stack:Stack<i32>  = super::Stack::with_capacity(5);
    stack.pop();
    stack.push(3);
    stack.pop();
    stack.pop();
    stack.push(2);
    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), None);
  }

  #[test]
  fn can_push_till_full() {
    let mut stack: Stack<i32> = Stack::with_capacity(2);
    stack.push(2);
    stack.push(2);
  }

  #[test]
  #[should_panic]
  fn can_not_push_after_full() {
    let mut stack: Stack<i32> = Stack::with_capacity(2);
    stack.push(2);
    stack.push(2);
    stack.push(2);
  }
}

