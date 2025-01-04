struct CircularBuffer<T>{
    items: Vec<T>,
    capacity: usize,
    oldest_element: usize
}

impl<T> CircularBuffer<T>{
    pub fn new(capacity: usize) -> Self{
        CircularBuffer{
            items: Vec::new(),
            capacity,
            oldest_element: 0
        }
    }

    pub fn push(&mut self, item: T){
        //Check if the buffer is full
        if self.items.len() == self.capacity{

            //If full...
            //Replace oldest element 
            self.items[self.oldest_element] = item;

            // update reference/pointer
            self.update_oldest_element();

        }else{
            //If not full
            self.items.push(item);
        }
        
    }

    pub fn pop(&mut self) -> T{
        let popped = self.items.remove(self.oldest_element);

        //Update pointer
        if self.oldest_element == self.capacity - 1{
            self.oldest_element = 0;
        }
        return popped;
    }

    pub fn size(&self) -> usize{
        self.items.len()
    }

    pub fn is_empty(&self) -> bool{
        self.items.is_empty()
    }

    fn update_oldest_element(&mut self){
        if self.oldest_element == self.capacity - 1{
            self.oldest_element = 0;
        }else{
            self.oldest_element = self.oldest_element + 1;
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]

mod tests{
    use super::*;

    #[test]

    fn non_replace_push(){
        let mut cbuf = CircularBuffer::new(5);
        cbuf.push(10);
        cbuf.push(20);
        assert_eq!(cbuf.size(), 2);
    }
    #[test]
    fn replacing_push(){
        let mut cbuf = CircularBuffer::new(1);
        cbuf.push(5);
        cbuf.push(4);
        cbuf.push(6);
        assert_eq!(cbuf.items[0], 6);
    }
    #[test]
    fn pop(){
        let mut cbuf = CircularBuffer::new(2);
        cbuf.push(34);
        cbuf.push(35);
        cbuf.push(36);
        assert_eq!(cbuf.items[1], 35);
        assert_eq!(cbuf.pop(),35);
    }
    #[test]
    fn is_empty(){
        let mut cbuf = CircularBuffer::new(1);
        assert_eq!(cbuf.is_empty(), true);
        cbuf.push(1);
        assert_eq!(cbuf.is_empty(), false);
    }
    #[test]
    fn size(){
        let mut cbuf = CircularBuffer::new(4);
        cbuf.push(2);
        assert_eq!(cbuf.size(), 1);
    }

}
