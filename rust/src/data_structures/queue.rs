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

    impl<T, const N: usize> Queue<T, N> {
        pub fn new() -> Self {
            Self {
                front: 0,
                rear: 0,
                items: [(); N].map(|_| None),
            }
        }

        pub fn enqueue(&mut self, item: T) -> Result<(), QueueError> {
            if self.rear == N {
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
            if self.front == self.rear {
                self.front = 0;
                self.rear = 0;
            }
            item
        }

        pub fn peek(&self) -> Option<&T> {
            self.items[self.front].as_ref()
        }

        pub fn is_empty(&self) -> bool {
            self.front == self.rear
        }

        pub fn is_full(&self) -> bool {
            self.rear == N
        }
    }
}

#[cfg(test)]
mod tests {

    use super::oo::{self, Queue};

    fn setup_queue() -> oo::Queue<String, 3> {
        let arr = ["first", "second", "third"];
        let mut queue = oo::Queue::<String, 3>::new();
        arr.iter().for_each(|s| queue.enqueue(s.to_string()).expect("failed to enque"));
        queue
    }

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
        let mut queue = setup_queue();

        assert_eq!(queue.dequeue(), Ok("first".to_string()))
    }

    #[test]
    fn test_oo_peek() {
        let mut queue = oo::Queue::<u8, 3>::new();
        queue.enqueue(3).expect("failed to enqueue");
        queue.enqueue(6).expect("failed to enqueue");

        assert_eq!(queue.peek(), Some(3).as_ref())
    }

    #[test]
    fn test_oo_is_empty() {
        let mut queue = Queue::<i32, 3>::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(2048).expect("failed to enque");
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_oo_is_full() {
        let mut queue = Queue::<i32, 3>::new();
        assert_eq!(queue.is_full(), false);
        queue.enqueue(2048).expect("failed to enque");
        assert_eq!(queue.is_full(), false);
        queue.enqueue(4096).expect("failed to enque");
        queue.enqueue(8192).expect("failed to enque");
        assert_eq!(queue.is_full(), true);
    }
}
