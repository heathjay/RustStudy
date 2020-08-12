use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};
enum List{
    Cons(i32, RefCell<Rc<List>>),
    Nil
}

impl List{
    fn tail(&self) -> Option<&RefCell<Rc<List>>>{
        match self{
            Cons(_,item) => Some(item),
            Nil => None,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
