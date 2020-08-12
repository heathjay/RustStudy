//创建一个用于存放其拥有所有权的i32的值和其子节点引用的node:
use std::rc::{Rc,Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node{
    value : i32,
    children: RefCell<Vec<Rc<Node>>>,

    //增加父节点
    parent: RefCell<Weak<Node>>,
}

fn main() {

    // clone了leaf中的Rc<Node>并存储在branch中，
    //leaf中的Node现在有两个所有者leaf和branch
    //可以通过branch.children从branch中获取leaf，不过无法从leaf到branch
    let leaf = Rc::new(Node{
        value:3,
        children:RefCell::new(vec![]),
        //刚开始没有父节点
        parent: RefCell::new(Weak::new()),
    });
    //upgrade获取leaf的父节点
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(
        Node{
            value:5,
            parent: RefCell::new(Weak::new()),
            children:RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    //可视化strong_count和weak_count的改变
    let leaf = Rc::new(Node{
        value:3,
        children:RefCell::new(vec![]),
        //刚开始没有父节点
        parent: RefCell::new(Weak::new()),
    });
    //leaf strong = {}, weak = {}
    //一旦创建leaf 强引用为1，弱引用为0
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    
    {
        let branch = Rc::new(
        Node{
            value:5,
            parent: RefCell::new(Weak::new()),
            children:RefCell::new(vec![Rc::clone(&leaf)]),
        }
    );
    //branch强引用1，弱引用1因为leaf.parent通过weak<Node>指向branch.
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("branch strong = {} weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    //leaf强引用为2，因为branch中的branch.children存储了leaf的Rc<Node>拷贝
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    //内部作用域结束，branch离开作用域，Rc的强引用为0，所以Node被丢弃，
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
