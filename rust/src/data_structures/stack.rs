mod oo {
    #[derive(Debug)]
    pub struct Stack<const N: usize> {
        items: [u8; N],
        top: usize,
    }

    impl<const N: usize> Stack<N> {
        pub const fn new() -> Self {
            Self {
                items: [0; N],
                top: 0,
            }
        }

        pub fn push(&mut self, value: u8) -> Result<(), &str> {
            if self.top + 1 > self.items.len() {
                return Err("Stack overflow");
            }
            self.items[self.top] = value;
            self.top += 1;
            Ok(())
        }

        pub fn pop(&mut self) -> Option<u8> {
            if self.top < 1 {
                return None;
            }
            self.top -= 1;
            Some(self.items[self.top])
        }

        pub fn is_empty(&self) -> bool {
            self.top == 0
        }

        pub fn is_full(&self) -> bool {
            self.top == self.items.len()
        }

        pub fn peek(&self) -> Option<u8> {
            if self.top == 0 {
                return None;
            }
            Some(self.items[self.top - 1])
        }
    }
}

mod f {
    pub fn stack_new<const N: usize>() -> (usize, [u8; N]) {
        (0, [0; N])
    }

    pub fn stack_push<const N: usize>(stack: &mut (usize, [u8; N]), value: u8) -> Result<(), &str> {
        let (top, items) = stack;
        if *top + 1 > items.len() {
            return Err("Stack overflow");
        }
        items[*top] = value;
        *top += 1;
        Ok(())
    }

    pub fn stack_pop<const N: usize>(stack: &mut (usize, [u8; N])) -> Option<u8> {
        let (top, items) = stack;
        if *top == 0 {
            return None;
        }
        *top -= 1;
        Some(items[*top])
    }

    pub fn stack_is_empty<const N: usize>(stack: &(usize, [u8; N])) -> bool {
        stack.0 == 0
    }

    pub fn stack_is_full<const N: usize>(stack: &(usize, [u8; N])) -> bool {
        stack.0 >= stack.1.len()
    }

    pub fn stack_peek<const N: usize>(stack: &(usize, [u8; N])) -> Option<u8> {
        if stack.0 == 0 {
            return None;
        }
        Some(stack.1[stack.0 -1])
    }
}

#[cfg(test)]
mod tests {
    use super::oo::Stack;
    use super::f::*;

    fn prepare_oo_stack() -> Stack<3> {
        let mut stack = Stack::new();
        (0..3).for_each(|i| {
            stack.push(i).expect("failed to push to test stack");
        });
        stack
    }

    #[test]
    fn test_oo_push() {
        let mut stack = prepare_oo_stack();
        assert_eq!(stack.peek(), Some(2));

        match stack.push(7) {
            Ok(()) => panic!("Expected an overflow"),
            Err(e) => assert_eq!(e, "Stack overflow"),
        }
    }

    #[test]
    fn test_oo_pop() {
        let mut stack = prepare_oo_stack();
        let poped = stack.pop();
        assert_eq!(poped, Some(2));
    }

    #[test]
    fn test_oo_is_empty() {
        let mut stack = Stack::<2>::new();
        assert_eq!(stack.is_empty(), true);

        stack.push(6).expect("failed to push");
        assert_eq!(stack.is_empty(), false);
    }

    #[test]
    fn test_oo_is_full() {
        let mut stack = prepare_oo_stack();
        assert_eq!(stack.is_full(), true);

        stack.pop();
        assert_eq!(stack.is_full(), false);
    }

    #[test]
    fn test_oo_peek() {
        let stack = prepare_oo_stack();
        assert_eq!(stack.peek(), Some(2));
    }

    #[test]
    fn test_f_push() {
        let mut stack = stack_new::<3>();
        stack_push(&mut stack, 6).expect("failed to push");
    }

    #[test]
    fn test_f_pop() {
        let mut stack = stack_new::<3>();
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_pop(&mut stack), Some(6));
    }

    #[test]
    fn test_f_is_empty() {
        let mut stack = stack_new::<3>();
        assert_eq!(stack_is_empty(&stack), true);
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_is_empty(&stack), false);
    }

    #[test]
    fn test_f_is_full() {
        let mut stack = stack_new::<1>();
        assert_eq!(stack_is_full(&stack), false);
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_is_full(&stack), true);
    }

    #[test]
    fn test_f_peek() {
        let mut stack = stack_new::<2>();
        stack_push(&mut stack, 6).expect("failed to push");
        assert_eq!(stack_peek(&stack), Some(6));
    }
}
