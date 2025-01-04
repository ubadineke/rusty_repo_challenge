pub struct Stack<T>{
    pub items: Vec<T>
}
  
impl<T> Stack<T>{
    pub fn new() -> Self{
        Stack { 
            items: Vec::new()
         }
    }
    //push
    pub fn push(&mut self, item: T){
        self.items.push(item);
    }
  
    //pop
    pub fn pop(&mut self) -> Option<T>{
        self.items.pop()
    }
  
    // peek
    pub fn peek(&self) -> Option<&T>{
        self.items.last()
    }
  
    // is_empty
    pub fn is_empty(&self) -> bool{
        if self.items.len() == 0{
             true  
        } else{
            false
        }
    }
  
    // size
    pub fn size(&self) -> usize {
        self.items.len()
    }
  }
  
  // Re-export Stack from main.rs (making it accessible)
// pub use self::Stack;
fn main() {

  }
  
    //Tests
#[cfg(test)]
mod stack{
    use super::*;

    #[test]
    fn new(){
        let mut stack = Stack::new();
        stack.push(4);
        assert_eq!(stack.size(), 1)
    }

  
    #[test]
    fn push(){
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.items.contains(&10), true);
    }
    
    #[test]
    fn pop(){
        let mut stack = Stack::new();
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.pop(), Some(15));
    }
    
    #[test]
    fn peek(){
        let mut stack = Stack::new();
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.peek(), Some(&15));  
    }
    
    #[test]
    fn is_empty(){
        let mut stack = Stack::new();
        assert_eq!(stack.is_empty(), true);  
        stack.push(20);
        assert_eq!(stack.is_empty(), false);  
    }
    
    #[test]
    fn size(){
        let mut stack = Stack::new();
        stack.push(20);
        stack.push(15);
        assert_eq!(stack.size(), 2);
    }
  }
  