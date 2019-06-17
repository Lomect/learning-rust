use std::cell::RefCell;
use std::rc::{Rc, Weak};
use List::{Nil, Cons};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parents: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a: {}", Rc::strong_count(&a));
    println!("a: {:?}", a.tail());

    let b = Rc::new(Cons(4, RefCell::new(Rc::clone(&a))));
    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));
    println!("b: {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("a: {}", Rc::strong_count(&a));
    println!("b: {}", Rc::strong_count(&b));

    let leaf = Rc::new(Node{
        value: 3,
        parents: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 4,
        parents: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parents.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf: {:?}", leaf.parents.borrow().upgrade());
    println!("leaf: weak: {}, strong: {}", Rc::weak_count(&leaf), Rc::strong_count(&leaf));
    println!("branch: weak: {}, strong: {}", Rc::weak_count(&branch), Rc::strong_count(&branch));
}
