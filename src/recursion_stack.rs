/// # Recursion Stack
/// This module provides a stack that can be filled and emptied recursionaly.
/// The stack is generic and can be used with any type that implements the
/// Clone trait.
/// # Example
/// ```
/// use recursion_stack::*;
/// let mut stack = Stack::new();
/// fill_stack_recursionaly(&mut stack, 10);
/// assert_eq!(stack.data, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
/// empty_stack_recursionaly(&mut stack);
/// assert_eq!(stack.data, vec![]);
/// ```
pub struct Stack<T> {
    data: Vec<T>,
}

/// StackCallbacks is a trait that allows to modify the behavior of the stack
/// when it is filled or emptied.
/// Default implementation is provided for the base_callback and modify_callback
/// methods. Default implementation returns true for base_callback and clones
/// the value for modify_callback.
/// You can override the default behavior by implementing the trait for your types.
/// # Example
/// ```
/// use recursion_stack::*;
/// impl StackCallbacks<i32> for Stack<i32> {
///    fn base_callback(&mut self, n: &i32) -> bool {
///       if *n == 0 {
///          true
///      } else {
///         false
///     }
///   }
///  fn modify_callback(&mut self, n: &i32) -> i32 {
///    n - 1
///   }
/// }
/// ```
pub trait StackCallbacks<T> {
    fn base_callback(&mut self, n: &T) -> bool {
        true
    }
    fn modify_callback(&mut self, n: &T) -> T where T: Clone {
        n.clone()   
    }
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }

    fn push(&mut self, value: T) {
        self.data.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub fn fill_stack_recursionaly<T: Clone>(stack: &mut Stack<T>, n: T) where Stack<T>: StackCallbacks<T> {
    if stack.base_callback(&n) {
        return;
    }
    let next = stack.modify_callback(&n);
    stack.push(n);
    fill_stack_recursionaly(stack, next);
}

pub fn empty_stack_recursionaly<T>(stack: &mut Stack<T>) {
    if stack.is_empty() {
        return;
    }
    stack.pop();
    empty_stack_recursionaly(stack);
}

#[cfg(test)]
mod tests {
    use super::*;

    impl StackCallbacks<i32> for Stack<i32> {
        fn base_callback(&mut self, n: &i32) -> bool {
            if *n == 0 {
                true
            } else {
                false
            }
        }

        fn modify_callback(&mut self, n: &i32) -> i32 {
            n - 1
        }
    }

    impl StackCallbacks<String> for Stack<String> {
        fn base_callback(&mut self, n: &String) -> bool {
            if n.is_empty() {
                true
            } else {
                false
            }
        }

        fn modify_callback(&mut self, n: &String) -> String {
            n.chars().skip(1).collect()
        }        
    }

    #[test]
    fn test_fill_stack_recursionaly() {
        let mut stack = Stack::new();
        fill_stack_recursionaly(&mut stack, 10);
        assert_eq!(stack.data, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_empty_stack_recursionaly() {
        let mut stack = Stack::new();
        fill_stack_recursionaly(&mut stack, 10);
        empty_stack_recursionaly(&mut stack);
        assert_eq!(stack.data, vec![]);
    }

    #[test]
    fn test_fill_stack_recursionaly_with_string() {
        let mut stack: Stack<String> = Stack::new();
        fill_stack_recursionaly(&mut stack, "Hello".to_string());
        assert_eq!(stack.data, vec!["Hello".to_string(), "ello".to_string(), "llo".to_string(), "lo".to_string(), "o".to_string()]);
    }
}
