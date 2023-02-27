use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Rc<Node<T>>>) -> Rc<Self> {
        Rc::new(Node { value, next })
    }
}

fn main() {
    let node1 = Node::new(1, None);
    let node2 = Node::new(2, Some(Rc::clone(&node1)));
    let node3 = Node::new(3, Some(Rc::clone(&node2)));
    println!("The value of node 1 is {}", node1.value);
    if let Some(node2_next) = &node2.next {
        println!("The value of node 2's next is {}", node2_next.value);
    }
    if let Some(node3_next) = &node3.next {
        println!("The value of node 3's next is {}", node3_next.value);
    }
}
