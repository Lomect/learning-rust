use std::cell::RefCell;

pub trait Messager {
    fn send(&self, msg: &str);
}

pub struct  LimitTracker<'a, T: 'a + Messager> {
    messager: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messager
{
    pub fn new(messager: &T, max: usize) -> LimitTracker<T> {
        LimitTracker{
            messager,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let per_of_max = self.value as f64/ self.max as f64;
        if per_of_max >= 0.75 && per_of_max < 0.9 {
            self.messager.send("You use per up 75%");
        } else if per_of_max >= 0.9 && per_of_max < 1.0 {
            self.messager.send("You use per up 90%");
        } else if per_of_max >1.0 {
            self.messager.send("Error You per is err");
        }
    }
}

/// 多个可变拥有者
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

/// cargo test -- --nocapture  # 可见测试过程中的打印结果
#[cfg(test)]
mod tests {
    use super::*;
    use core::borrow::Borrow;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct  MockMessager {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> Self {
            MockMessager{ send_messages: RefCell::new(vec![])}
        }
    }

    impl Messager for MockMessager {
        fn send(&self, msg: &str) {
            self.send_messages.borrow_mut().push(String::from(msg));
        }
    }
    #[test]
    fn it_works() {
        let mock_messges = MockMessager::new();
        let mut lim = LimitTracker::new(&mock_messges, 100);
        lim.set_value(77);
        println!("{:?}", mock_messges.borrow());
        assert_eq!(mock_messges.send_messages.borrow().len(), 1);
    }

    #[test]
    fn many_mut() {
        let value = Rc::new(RefCell::new(5));
        let a  = Rc::new(List::Cons(Rc::clone(&value),Rc::new(List::Nil)));
        let b = List::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = List::Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

        *value.borrow_mut() = 12;
        println!("a: {:?}, \nb: {:?}, \nc: {:?}",a, b, c);
    }
}
