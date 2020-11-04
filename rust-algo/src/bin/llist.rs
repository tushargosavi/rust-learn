use std::mem;

#[derive(Debug)]
pub struct List {
    head : Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let node = Box::new(Node{
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty)
        });

        self.head = Link::More(node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(ref mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty)
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