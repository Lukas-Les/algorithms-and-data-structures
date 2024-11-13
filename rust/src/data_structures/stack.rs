use std::u8;


#[derive(Debug)]
struct Stack <const N: usize> {
    items: [u8; N], 
    top: usize,
}

impl <const N: usize> Stack <N> {
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
        let top_item_idx = self.top - 1;
        let top_item: u8 = self.items[top_item_idx];
        self.items[top_item_idx] = 0;
        self.top -= 1;
        Some(top_item)
    }
}


mod tests {
    use super::*;
    
    fn prepare_stack() -> Stack<3> {
        let mut stack = Stack::new();
        (0..3).for_each(|i| {
            stack.push(i).expect("failed to push to test stack");
            }
        );
        stack
    }

    #[test]
    fn test_push() {
        let mut stack = prepare_stack();
        assert_eq!(stack.items[0], 0);

        match stack.push(7) {
            Ok(()) => panic!("Expected an overflow"),
            Err(e) => assert_eq!(e, "Stack overflow"),
        }
    }

    #[test]
    fn test_pop() {
        let mut stack = prepare_stack();
        let poped = stack.pop();
        assert_eq!(poped, Some(2));
    }

}
