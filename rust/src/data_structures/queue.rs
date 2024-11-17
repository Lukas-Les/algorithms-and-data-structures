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

mod f {
    type Queue<T, const N: usize> = (usize, usize, [Option<T>; N]);

    pub fn queue_new<T, const N: usize>() -> Queue<T, N> {
        // 0 - front
        // 1- rear
        // 2 - items
        (0, 0, [(); N].map(|_| None))
    }

    pub fn queue_enqueue<T, const N: usize>(queue: &mut Queue<T, N>, item: T) -> Result<(), &str> {
        if queue.1 == N {
            return Err("Queue is full");
        }
        queue.2[queue.1] = Some(item);
        queue.1 += 1;
        Ok(())
    }

    pub fn queue_dequeue<T, const N: usize>(queue: &mut Queue<T, N>) -> Option<T> {
        if queue.0 == queue.1 {
            return None;
        }
        let item = queue.2[queue.0].take();
        queue.0 += 1;
        if queue.0 == N {
            queue.0 = 0;
            queue.1 = 0;
        }
        item
    }

    pub fn queue_peek<'a, T, const N: usize>(queue: &'a Queue<T, N>) -> Option<&'a T> {
        queue.2[queue.0].as_ref()
    }

    pub fn queue_is_empty<T, const N: usize>(queue: &Queue<T, N>) -> bool {
        queue.0 == queue.1
    }

    pub fn queue_is_full<T, const N: usize>(queue: &Queue<T, N>) -> bool {
        queue.1 == N
    }
}

#[cfg(test)]
mod tests {

    use super::f;
    use super::oo;
    fn setup_queue() -> oo::Queue<String, 3> {
        let arr = ["first", "second", "third"];
        let mut queue = oo::Queue::<String, 3>::new();
        arr.iter()
            .for_each(|s| queue.enqueue(s.to_string()).expect("failed to enque"));
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
        let mut queue = oo::Queue::<i32, 3>::new();
        assert_eq!(queue.is_empty(), true);
        queue.enqueue(2048).expect("failed to enque");
        assert_eq!(queue.is_empty(), false);
    }

    #[test]
    fn test_oo_is_full() {
        let mut queue = oo::Queue::<i32, 3>::new();
        assert_eq!(queue.is_full(), false);
        queue.enqueue(2048).expect("failed to enque");
        assert_eq!(queue.is_full(), false);
        queue.enqueue(4096).expect("failed to enque");
        queue.enqueue(8192).expect("failed to enque");
        assert_eq!(queue.is_full(), true);
    }

    #[test]
    fn test_f_enqueue() {
        let mut queue = f::queue_new::<u8, 3>();
        f::queue_enqueue(&mut queue, 6).expect("failed to enqueue");
        f::queue_enqueue(&mut queue, 6).expect("failed to enqueue");
        f::queue_enqueue(&mut queue, 6).expect("failed to enqueue");

        match f::queue_enqueue(&mut queue, 6) {
            Ok(()) => panic!("expected queue to be full"),
            Err(e) => assert_eq!(e, "Queue is full"),
        }
    }

    #[test]
    fn test_f_dequeue() {
        let mut queue = f::queue_new::<u8, 3>();
        f::queue_enqueue(&mut queue, 6).expect("failed to enqueue");

        assert_eq!(f::queue_dequeue(&mut queue), Some(6));
    }

    #[test]
    fn test_f_peek() {
        let mut queue = f::queue_new::<String, 2>();
        f::queue_enqueue(&mut queue, "first".to_string()).expect("failed to enqueue");
        f::queue_enqueue(&mut queue, "second".to_string()).expect("failed to enqueue");
        let want = "first".to_string();
        assert_eq!(f::queue_peek(&queue), Some(&want));
    }

    #[test]
    fn test_f_is_empty() {
        let mut queue = f::queue_new::<String, 2>();
        assert_eq!(f::queue_is_empty(&queue), true);
        f::queue_enqueue(&mut queue, "first".to_string()).expect("failed to enqueue");
        f::queue_enqueue(&mut queue, "second".to_string()).expect("failed to enqueue");
        assert_eq!(f::queue_is_empty(&queue), false);
    }

    #[test]
    fn test_f_is_full() {
        let mut queue = f::queue_new::<String, 2>();
        f::queue_enqueue(&mut queue, "first".to_string()).expect("failed to enqueue");
        assert_eq!(f::queue_is_full(&queue), false);
        f::queue_enqueue(&mut queue, "second".to_string()).expect("failed to enqueue");
        assert_eq!(f::queue_is_full(&queue), true);
    }
}
