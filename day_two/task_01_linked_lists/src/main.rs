use std::fmt::Display;

struct Node <T>{
    data: T,
    next: Option<Box<Node<T>>>
}


struct LinkedList<T>{
    head: Option<Box<Node<T>>>
}

impl<T: Display> LinkedList<T>{
    pub fn new() -> Self{
        LinkedList {head: None}
    }

    pub fn push(& mut self, data: T){
        let new_node = Box::new(Node {
            data,
            next: self.head.take()
        });
        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<T>{
        self.head.take().map(|existing_head|{
            self.head = existing_head.next;

            existing_head.data
        })
    }

    pub fn display(&self) {
        let mut current = &self.head;

        while let Some(node) = current{
            print!("{} -> ", node.data );
            current = &node.next
        }

        print!("None");
    }

    pub fn is_empty(&self) -> bool{
        self.head.is_none()
    }

    pub fn peek(&self) -> Option<&T>{
        self.head.as_ref().map(|node| &node.data)
    }
}
fn main() {
    println!("Hello, world!");
    let mut linked_list = LinkedList::new();
    linked_list.push(90);
    linked_list.push(80);
    linked_list.display();
}

#[cfg(test)]

mod tests{
    use super::*;
    #[test]
    fn new(){
        let linked_list = LinkedList::<i32>::new();
        assert_eq!(linked_list.is_empty(), true)
    }

    #[test]
    fn push(){
        let mut linked_list = LinkedList::new();
        linked_list.push(90);

        assert_eq!(linked_list.peek(), Some(&90))
    }

    #[test]
    fn pop(){
        let mut linked_list = LinkedList::new();
        linked_list.push(90);
        linked_list.push(80);
        linked_list.push(70);
        linked_list.display();
        linked_list.pop();
        linked_list.display();

        assert_eq!(linked_list.pop(), Some(80))

    }
}
