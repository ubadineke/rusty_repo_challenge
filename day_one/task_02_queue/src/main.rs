use task_01_stack::Stack;

pub struct Queue<T>{
    pub enqueue_stack: Stack<T>,
    pub dequeue_stack: Stack<T>
}

impl<T> Queue<T>{
    fn new() -> Self{
        Queue{
            enqueue_stack: Stack::new(),
            dequeue_stack: Stack::new()
        }
    }

    fn enqueue(&mut self, item: T) {
        self.enqueue_stack.push(item)
    }

    fn dequeue(&mut self) -> Option<T>{
        if self.dequeue_stack.is_empty(){
            while let Some(item) = self.enqueue_stack.pop(){
                self.dequeue_stack.push(item);
            }
        }
        self.dequeue_stack.pop()
    }

    fn is_empty(&self) -> bool {
    // ) && (self.dequeue_stack.is_empty())
        if self.enqueue_stack.is_empty() && self.dequeue_stack.is_empty(){
            return true
        }else{
            false   
        }
    }

    fn size(&self) -> usize {
        self.enqueue_stack.size() + self.dequeue_stack.size()
    }
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod queues{
    use super::*;

    #[test]
    fn new(){
        let mut queue = Queue::new();
        queue.enqueue(4);
        assert_eq!(queue.size(), 1)
    }

    #[test]
    fn enqueue(){
        let mut queue = Queue::new();
        queue.enqueue(8);
        assert_eq!(queue.size(), 1);
    }

    #[test]
    fn dequeue(){
        let mut queue = Queue::new();
        queue.enqueue(80);
        assert_eq!(queue.dequeue(), Some(80));
    }

    #[test]
    fn is_empty(){
        let mut queue = Queue::new();
        assert_eq!(queue.is_empty(), true);  
        queue.enqueue(20);
        assert_eq!(queue.is_empty(), false);  
    }

    #[test]
    fn size(){
        let mut queue = Queue::new();
        queue.enqueue(40);
        queue.enqueue(30);
        queue.enqueue(30);
        assert_eq!(queue.size(), 3)
    }

}


