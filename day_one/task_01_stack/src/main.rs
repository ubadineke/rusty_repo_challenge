pub struct Stack<T>{
    items: Vec<T>
}
  
impl<T> Stack<T>{
    //push
    fn push(&mut self, item: T){
        self.items.push(item);
    }
  
    //pop
    fn pop(&mut self) -> Option<T>{
        self.items.pop()
    }
  
    // peek
    fn peek(&self) -> Option<&T>{
        self.items.last()
    }
  
    // is_empty
    fn is_empty(&self) -> bool{
        if self.items.len() == 0{
             true  
        } else{
            false
        }
    }
  
    // size
    fn size(&self) -> usize {
        self.items.len()
    }
  }
  
fn main() {

  }
  
    //Tests
#[cfg(test)]
mod tests{
    use super::*;
  
    #[test]
    fn push(){
        let mut stack = Stack{ items: Vec::new()};
        stack.push(10);
        assert_eq!(stack.items.contains(&10), true);
        println!("Push stack operation succesfull");
    }
    
    #[test]
    fn pop(){
        let mut stack = Stack{items: Vec::new()};
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.pop(), Some(15));
    }
    
    #[test]
    fn peek(){
        let mut stack = Stack{items: Vec::new()};
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.peek(), Some(&15));  
    }
    
    #[test]
    fn is_empty(){
        let mut stack = Stack{items: Vec::new()};
        assert_eq!(stack.is_empty(), true);  
        stack.push(20);
        assert_eq!(stack.is_empty(), false);  
    }
    
    #[test]
    fn size(){
        let mut stack = Stack{items: Vec::new()};
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.size(), 2);
    }
  }
  