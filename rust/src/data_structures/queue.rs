mod oo {
    #[derive(Debug, PartialEq)]
    pub enum QueueError {
        Full,
        Empty,
    }

    pub struct Queue<T, const N: usize> {
        front: usize,
        rear: usize,
        items: [Option<T>; N],
    }
    
    impl <T, const N: usize> Queue<T, N> {
        pub fn new() -> Self {
            Self {
                front: 0,
                rear: 0,
                items: [(); N].map(|_| None),
            }
        }

        pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
            if self.front == 0 {
                self.front = 1;
            }
            if self.rear == self.items.len() {
                return Err(QueueError::Full);
            }
            self.items[self.rear] = Some(item);
            self.rear += 1;
            
            Ok(())
        }
        pub fn dequeue(&mut self) -> Result<T, QueueError> {
            if self.front == self.rear {
                return Err(QueueError::Empty);
            }
            let item = self.items[self.front].take().ok_or(QueueError::Empty);
            self.front += 1;
            if self.front == self.items.len() {
                self.front = 0;
                self.rear = 0;
            }
            item
        }
    }
}

#[cfg(test)]
mod tests {

    use super::oo; 

    #[test]
    fn test_oo_enqueue() {
        let mut queue = oo::Queue::<u8, 3>::new();
        queue.enqueue(6).expect("failed to enqueue");
        queue.enqueue(6).expect("failed to enqueue");
        queue.enqueue(6).expect("failed to enqueue");
        
        match queue.enqueue(6) {
            Ok(()) => panic!("expected queue to be full"),
            Err(e) => assert_eq!(e, oo::QueueError::Full),
        }
    }

    #[test]
    fn test_oo_dequeue() {
        let mut queue = oo::Queue::<u8, 3>::new();
        queue.enqueue(6).expect("failed to enqueue");
        queue.enqueue(6).expect("failed to enqueue");
        
        assert_eq!(queue.dequeue(), Ok(6))
    }
}
