use task_01_linked_lists::LinkedList;

pub trait LinkedListExt<T>{
    fn reverse(&mut self);
}

impl<T> LinkedListExt<T> for LinkedList<T>{
     fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();
    
        while let Some(mut node) = current{
            let next = node.next.take();
            node.next = prev;
            prev = Some(node);
            current = next;
        }
        self.head = prev;
    }

}

fn main() {
    println!("Hello, world!");
        let mut linked_list = LinkedList::new();
        linked_list.push(10);
        linked_list.push(20);
        linked_list.push(30);
        linked_list.push(40);
        linked_list.display();
        linked_list.reverse();
        println!("Reversed List");
        linked_list.display();
    
}


#[cfg(test)]

mod tests{
    use super::*;

    #[test]
    fn reverse(){
        let mut linked_list = LinkedList::new();
        linked_list.push(10);
        linked_list.push(20);
        linked_list.push(30);
        linked_list.push(40);
        linked_list.display();
        linked_list.reverse();
        linked_list.display();
        
        assert_eq!(linked_list.peek(), Some(&10));
    }
}