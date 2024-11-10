use std::u8;

const SIZE: usize = 3;


struct Stack {
    items: [u8; SIZE], 
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            items: [0; SIZE],
            top: 0,
        }
    }

    pub fn push(&mut self, value: u8) -> Result<(), &str> {
        if self.top >= SIZE {
            return Err("Stack overflow");
        }
        self.top += 1;
        self.items[self.top - 1] = value;
        Ok(())
    }

}


mod tests {
    use super::*;
    
    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        for i in 0..SIZE {
            stack.push(i as u8);
        }
        assert_eq!(stack.items[0], 0);

        match stack.push(7) {
            Ok(()) => panic!("Expected an overflow"),
            Err(e) => assert_eq!(e, "Stack overflow"),
        }
    }

}
