mod oo {

    #[derive(Debug, PartialEq)]
    pub enum StackError {
        Overflow,
        Underflow,
    }

    pub struct Stack<T, const N: usize> {
        items: [Option<T>; N],
        top: usize,
    }

    impl<T, const N: usize> Stack<T, N> {
        pub fn new() -> Self {
            Self {
                items: [(); N].map(|_| None),
                top: 0,
            }
        }

        pub fn push(&mut self, value: T) -> Result<(), StackError> {
            if self.top >= self.items.len() {
                return Err(StackError::Overflow);
            }
            self.items[self.top] = Some(value);
            self.top += 1;
            Ok(())
        }

        pub fn pop(&mut self) -> Result<T, StackError> {
            if self.top < 1 {
                return Err(StackError::Underflow);
            }
            self.top -= 1;
            self.items[self.top].take().ok_or(StackError::Underflow)
        }

        pub fn is_empty(&self) -> bool {
            self.top == 0
        }

        pub fn is_full(&self) -> bool {
            self.top == self.items.len()
        }

        pub fn peek(&self) -> Option<&T> {
            if self.top == 0 {
                return None;
            }
            self.items[self.top - 1].as_ref()
        }
    }
}

mod f {

    type Stack<T, const N: usize> = (usize, [Option<T>; N]);

    pub fn stack_new<T, const N: usize>() -> Stack<T, N> {
        // First tuple item holds top position,
        // second holds items wrapped into Option type.
        (0, [(); N].map(|_| None))
    }

    pub fn stack_push<T, const N: usize>(stack: &mut Stack<T, N>, value: T) -> Result<(), &str> {
        let (top, items) = stack;
        if *top + 1 > items.len() {
            return Err("Stack is full");
        }
        items[*top] = Some(value);
        *top += 1;
        Ok(())
    }

    pub fn stack_pop<T, const N: usize>(stack: &mut Stack<T, N>) -> Option<T> {
        let (top, items) = stack;
        if *top == 0 {
            return None;
        }
        *top -= 1;
        items[*top].take()
    }

    pub fn stack_is_empty<T, const N: usize>(stack: &Stack<T, N>) -> bool {
        stack.0 == 0
    }

    pub fn stack_is_full<T, const N: usize>(stack: &Stack<T, N>) -> bool {
        stack.0 >= stack.1.len()
    }

    pub fn stack_peek<T, const N: usize>(stack: &Stack<T, N>) -> Option<&T> {
        if stack.0 == 0 {
            return None;
        }
        stack.1[stack.0 - 1].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::data_structures::stack::oo::StackError;

    use super::f::*;
    use super::oo::Stack;

    fn prepare_oo_stack() -> Stack<u8, 3> {
        let mut stack = Stack::<u8, 3>::new();
        (0..3).for_each(|i| {
            stack.push(i).expect("failed to push to test stack");
        });
        stack
    }

    #[test]
    fn test_oo_push() {
        let mut stack = prepare_oo_stack();
        assert_eq!(stack.peek(), Some(2).as_ref());

        match stack.push(7) {
            Ok(()) => panic!("Expected an overflow"),
            Err(e) => assert_eq!(e, StackError::Overflow),
        }
    }

    #[test]
    fn test_oo_pop() {
        let mut stack = prepare_oo_stack();
        let poped = stack.pop();
        assert_eq!(poped, Ok(2));
    }

    #[test]
    fn test_oo_is_empty() {
        let mut stack = Stack::<u8, 2>::new();
        assert_eq!(stack.is_empty(), true);

        stack.push(6).expect("failed to push");
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn test_oo_is_full() {
        let mut stack = prepare_oo_stack();
        assert_eq!(stack.is_full(), true);

        stack.pop().expect("failed to pop");
        assert_eq!(stack.is_full(), false);
    }

    #[test]
    fn test_oo_peek() {
        let stack = prepare_oo_stack();
        assert_eq!(stack.peek(), Some(2).as_ref());
    }

    #[test]
    fn test_f_push() {
        let mut stack = stack_new::<u8, 3>();
        stack_push(&mut stack, 6).expect("failed to push");
    }

    #[test]
    fn test_f_pop() {
        let mut stack = stack_new::<u8, 3>();
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_pop(&mut stack), Some(6));
    }

    #[test]
    fn test_f_is_empty() {
        let mut stack = stack_new::<u8, 3>();
        assert_eq!(stack_is_empty(&stack), true);
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_is_empty(&stack), false);
    }

    #[test]
    fn test_f_is_full() {
        let mut stack = stack_new::<u8, 1>();
        assert_eq!(stack_is_full(&stack), false);
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_is_full(&stack), true);
    }

    #[test]
    fn test_f_peek() {
        let mut stack = stack_new::<u8, 2>();
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_peek(&stack), Some(6).as_ref());
    }
}
