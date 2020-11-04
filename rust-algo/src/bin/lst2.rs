use std::mem;

#[derive(Debug)]
pub struct List<T> {
    head : Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node{
            elem: elem,
            next: self.head.take()
        });

        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(ref mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None)
        }
    }
}

fn main() {
    let mut lst = List::new();
    lst.push(10);
    lst.push(20);
    println!("lst {:?}", lst);
    lst.pop();
    println!("lst {:?}", lst);
}

#[cfg(test)]
mod test {
    #[test]
    fn basic() {
        let mut lst = super::List::new();
        
        assert_eq!(lst.pop(), None);

        lst.push(10);
        lst.push(20);
        lst.push(30);

        assert_eq!(lst.pop(), Some(30));
        assert_eq!(lst.pop(), Some(20));

        lst.push(40);
        lst.push(50);

        assert_eq!(lst.pop(), Some(50));
        assert_eq!(lst.pop(), Some(40));

        assert_eq!(lst.pop(), Some(10));
        assert_eq!(lst.pop(), None);
    }
}